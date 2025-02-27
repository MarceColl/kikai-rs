use bevy::prelude::*;
use rusqlite::{params, Connection, OpenFlags};
use rusqlite_migration::{Migrations, M};

pub struct UnitRepoPlugin;

#[derive(Resource)]
pub struct UnitRepository {
    pool: rusqlite_pool::ConnectionPool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct UnitDefinition {
    pub unit_id: u64,
    pub name: String,
    pub code: Option<String>,
}

impl UnitRepository {
    pub fn new(db_path: impl AsRef<std::path::Path> + Clone) -> Self {
        UnitRepository {
            pool: rusqlite_pool::ConnectionPool::new(10, || {
                Connection::open_with_flags(db_path.clone(), OpenFlags::default())
            })
            .unwrap(),
        }
    }

    pub fn get_connection(&self) -> Option<rusqlite_pool::ConnectionHandle> {
        self.pool.pop()
    }

    pub fn get_units(&self) -> Vec<UnitDefinition> {
        let mut conn = self.pool.pop().unwrap();
        let mut stmt = conn.prepare(
            "SELECT u.unit_id, u.name, uv.code FROM units AS u LEFT JOIN unit_versions AS uv ON (u.current_version_id = uv.version_id)"
        ).unwrap();

        let rows = stmt
            .query_map([], |row| {
                Ok(UnitDefinition {
                    unit_id: row.get(0)?,
                    name: row.get(1)?,
                    code: row.get(2)?,
                })
            })
            .unwrap();

        let mut units = Vec::new();

        for row in rows {
            units.push(row.unwrap());
        }

        units
    }

    pub fn new_unit_type(&self, name: String) {
        let mut conn = self.pool.pop().unwrap();
        conn.execute("INSERT INTO units (name) VALUES (?1)", [name])
            .unwrap();
    }

    pub fn update_code_for_unit(&self, unit_id: u64, new_code: String) {
        let mut conn = self.pool.pop().unwrap();
        let version_id: u64 = conn
            .query_row(
                "INSERT INTO unit_versions (unit_id, code) VALUES (?1, ?2) RETURNING version_id",
                (unit_id, new_code),
                |row| row.get(0),
            )
            .unwrap();
        conn.execute(
            "UPDATE units SET current_version_id = ?1 WHERE unit_id = ?2",
            [version_id, unit_id],
        )
        .unwrap();
    }
}

fn run_migrations(conn: &mut Connection) {
    let migrations = Migrations::new(vec![M::up(
        r#"
            -- Units table - stores basic unit information and tracks current version
            CREATE TABLE units (
                unit_id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                current_version_id INTEGER,
                FOREIGN KEY (current_version_id) REFERENCES unit_versions (version_id)
            );

            -- Unit versions table - stores different code versions for each unit
            CREATE TABLE unit_versions (
                version_id INTEGER PRIMARY KEY,
                unit_id INTEGER NOT NULL,
                code TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (unit_id) REFERENCES units (unit_id)
            );
        "#,
    )]);

    migrations.to_latest(conn).unwrap();
    println!("MIGRATIONS RAN");
}

fn setup_database(mut unit_repository: ResMut<UnitRepository>) {
    let mut conn = unit_repository.get_connection().unwrap();
    run_migrations(&mut conn);
}

impl Plugin for UnitRepoPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UnitRepository::new("./units.db"))
            .add_systems(Startup, setup_database);
    }
}
