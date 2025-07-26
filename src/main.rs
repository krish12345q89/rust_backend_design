#[cfg(windows)]
#[link(name = "advapi32")]
unsafe extern "system" {}
use actix_web::{web, App, HttpServer};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use chrono::NaiveDate;
use heed::{ Database, Env, EnvOpenOptions, RwTxn, RoTxn };
mod routes;
use heed::types::{ SerdeBincode, Str };
mod r#struct;
use crate::r#struct::{
    Products,
    Procurement,
    Procurements,
    ProductionRate,
    RecorderPoint,
    Components,
    Movements,
    Orders,
    SuppliersOrders,
    AssemblyTimeLine,
    Watches,
};

// ========== DATABASE IMPLEMENTATION ==========

pub struct InventoryDB {
    env: Env,
    products_db: Database<Str, SerdeBincode<Products>>,
    components_db: Database<Str, SerdeBincode<Components>>,
    movements_db: Database<Str, SerdeBincode<Movements>>,
    suppliers_orders_db: Database<Str, SerdeBincode<SuppliersOrders>>,
    orders_db: Database<Str, SerdeBincode<Orders>>,
    procurements_db: Database<Str, SerdeBincode<Procurements>>,
    assembly_timeline_db: Database<Str, SerdeBincode<AssemblyTimeLine>>,
    production_rate_db: Database<Str, SerdeBincode<ProductionRate>>,
    recorder_point_db: Database<Str, SerdeBincode<RecorderPoint>>,
    watches_db: Database<Str, SerdeBincode<Watches>>,
}

impl InventoryDB {
    pub fn new(path: &Path) -> Result<Self, heed::Error> {
        if !path.exists() {
            std::fs::create_dir_all(path)?;
        }

        let env = unsafe {
            EnvOpenOptions::new()
                .map_size(1024 * 1024 * 1024) // 1GB
                .max_dbs(10)
                .open(path)?
        };

        let mut wtxn = env.write_txn()?;

        let dbs = (
            env.create_database(&mut wtxn, Some("products"))?,
            env.create_database(&mut wtxn, Some("components"))?,
            env.create_database(&mut wtxn, Some("movements"))?,
            env.create_database(&mut wtxn, Some("suppliers_orders"))?,
            env.create_database(&mut wtxn, Some("orders"))?,
            env.create_database(&mut wtxn, Some("procurements"))?,
            env.create_database(&mut wtxn, Some("assembly_timeline"))?,
            env.create_database(&mut wtxn, Some("production_rate"))?,
            env.create_database(&mut wtxn, Some("recorder_point"))?,
            env.create_database(&mut wtxn, Some("watches"))?,
        );

        wtxn.commit()?;

        Ok(Self {
            env,
            products_db: dbs.0,
            components_db: dbs.1,
            movements_db: dbs.2,
            suppliers_orders_db: dbs.3,
            orders_db: dbs.4,
            procurements_db: dbs.5,
            assembly_timeline_db: dbs.6,
            production_rate_db: dbs.7,
            recorder_point_db: dbs.8,
            watches_db: dbs.9,
        })
    }

    // ========== TRANSACTION HELPERS ==========
    pub fn with_write_txn<F, T>(&self, f: F) -> Result<T, heed::Error>
        where F: FnOnce(&mut RwTxn) -> Result<T, heed::Error>
    {
        let mut wtxn = self.env.write_txn()?;
        let result = f(&mut wtxn)?;
        wtxn.commit()?;
        Ok(result)
    }

    pub fn with_read_txn<F, T>(&self, f: F) -> Result<T, heed::Error>
        where F: FnOnce(&RoTxn) -> Result<T, heed::Error>
    {
        let rtxn = self.env.read_txn()?;
        f(&rtxn)
    }

    // ========== SAMPLE DATA INITIALIZATION ==========
    pub fn initialize_sample_data(&self) -> Result<(), heed::Error> {
        self.with_write_txn(|wtxn| {
            // Clear all databases first
            self.products_db.clear(wtxn)?;
            self.components_db.clear(wtxn)?;
            self.movements_db.clear(wtxn)?;
            self.suppliers_orders_db.clear(wtxn)?;
            self.orders_db.clear(wtxn)?;
            self.procurements_db.clear(wtxn)?;
            self.assembly_timeline_db.clear(wtxn)?;
            self.production_rate_db.clear(wtxn)?;
            self.recorder_point_db.clear(wtxn)?;
            self.watches_db.clear(wtxn)?;

            // Create sample products
            let product_bp = Products {
                product_name: "BP Watch".to_string(),
                product_id: "PROD-001".to_string(),
                components: Some(vec!["COMP-001".to_string(), "COMP-002".to_string()]),
                cn: 100,
                kling: 50,
                st_jacob: 75,
                wurenlos: 200,
                wurenlos_sold: 25,
                flf: 150,
                in_transit: 30,
                total_available: 600,
                reserver_for_orders: 150,
                waste: 5,
                customer: 0,
            };
            self.products_db.put(wtxn, "PROD-001", &product_bp)?;

            // Create sample components
            let components = vec![
                Components {
                    product_id: "PROD-001".to_string(),
                    product_name: "BP Watch".to_string(),
                    component_name: "Premium Dial".to_string(),
                    component_id: "COMP-001".to_string(),
                    cn: 40,
                    kling: 20,
                    st_jacob: 30,
                    wurenlos: 80,
                    wurenlos_sold: 10,
                    flf: 60,
                    in_transit: 15,
                    total_available: 245,
                    ordered_surplus: 25.5,
                    reserver_for_orders: 60,
                    waste: 2,
                    customer: 0,
                    assembly_line: 0,
                },
                Components {
                    product_id: "PROD-001".to_string(),
                    product_name: "BP Watch".to_string(),
                    component_name: "Luminous Hands".to_string(),
                    component_id: "COMP-002".to_string(),
                    cn: 35,
                    kling: 15,
                    st_jacob: 25,
                    wurenlos: 70,
                    wurenlos_sold: 8,
                    flf: 50,
                    in_transit: 12,
                    total_available: 215,
                    ordered_surplus: 18.0,
                    reserver_for_orders: 45,
                    waste: 1,
                    customer: 0,
                    assembly_line: 0,
                }
            ];

            for component in components {
                self.components_db.put(wtxn, &component.component_id, &component)?;
            }

            // Create sample movements
            let movement_1 = Movements {
                movement_id: "MOVE-001".to_string(),
                transaction_id: "TRANS-001".to_string(),
                date: NaiveDate::from_ymd_opt(2023, 5, 15).expect("Invalid date"),
                movement_type: "Component".to_string(),
                component_name: Some("Premium Dial".to_string()),
                product_name: None,
                source_location: "CN".to_string(),
                destination_location: "Wurenlos".to_string(),
                quantity: 10,
                notes: Some("Regular stock transfer".to_string()),
                status: "Completed".to_string(),
                supplier_order_id: None,
            };
            self.movements_db.put(wtxn, "MOVE-001", &movement_1)?;

            // Create sample supplier order
            let supplier_order = SuppliersOrders {
                supplier_id: "SUPP-001".to_string(),
                component_name: "Sapphire Crystal".to_string(),
                procurement_id: "PROC-001".to_string(),
                order_id: "SUPP-ORD-001".to_string(),
                total_components_required: 50,
                components_roundof: 50,
                status: "Pending".to_string(),
                order_date: NaiveDate::from_ymd_opt(2023, 5, 15).expect("Invalid date"),
                expected_delivery_date: NaiveDate::from_ymd_opt(2023, 5, 15).expect("Invalid date"),
            };
            self.suppliers_orders_db.put(wtxn, "SUPP-ORD-001", &supplier_order)?;

            // Create sample order
            let customer_order = Orders {
                order_id: "ORD-001".to_string(),
                procurements: Some(vec!["PROC-001".to_string()]),
                supplier_orders: Some(vec!["SUPP-ORD-001".to_string()]),
                quanity_ordered: 50,
                product_id: "PROD-001".to_string(),
                product: "BP Watch".to_string(),
                quantity_required: 50,
                expected_delivery_date: NaiveDate::from_ymd_opt(2023, 5, 15).expect("Invalid date"),
                production_start_date: NaiveDate::from_ymd_opt(2023, 5, 15).expect("Invalid date"),
                expected_ship_date: NaiveDate::from_ymd_opt(2023, 5, 15).expect("Invalid date"),
                recid: "REC-001".to_string(),
                order_status: "Processing".to_string(),
                total_components_booked: 100,
                components_notes: Some("Need expedited shipping".to_string()),
                components_required: 100,
                total_gap_components: Some(vec![20, 30]),
                components: Some(vec!["COMP-001".to_string(), "COMP-002".to_string()]),
            };
            self.orders_db.put(wtxn, "ORD-001", &customer_order)?;

            // Create sample procurement
            // When creating sample data:
            let procurements = Procurements {
                procurement_id: "PROC-GROUP-001".to_string(),
                order_id: "ORD-001".to_string(),
                procurements: vec![Procurement {
                    procurement_id: "PROC-001".to_string(),
                    order_id: "ORD-001".to_string(),
                    components: Some(vec!["COMP-001".to_string()]),
                    quantity: 20,
                    status: "Pending".to_string(),
                    product: "BP Watch".to_string(),
                }],
            };
            self.procurements_db.put(wtxn, "PROC-GROUP-001", &procurements)?;

            // Create sample assembly timeline
            let assembly = AssemblyTimeLine {
                assembly_id: "ASSEM-001".to_string(),
                order: "ORD-001".to_string(),
                product: "BP Watch".to_string(),
                movements: vec!["MOVE-001".to_string()],
                components_required: 100,
                total_components_booked: 80,
                components: vec!["COMP-001".to_string(), "COMP-002".to_string()],
                total_gap_components: Some(vec![20]),
                assembly_location: "Wurenlos".to_string(),
                components_received_date: NaiveDate::from_ymd_opt(2023, 5, 15).expect(
                    "Invalid date"
                ),
                assembly_start_date: NaiveDate::from_ymd_opt(2023, 5, 15).expect("Invalid date"),
                assembly_end_date: NaiveDate::from_ymd_opt(2023, 5, 15).expect("Invalid date"),
                assembly_status: "Scheduled".to_string(),
                total_duration: 5,
                assembly_notes: Some("Priority order".to_string()),
            };
            self.assembly_timeline_db.put(wtxn, "ASSEM-001", &assembly)?;

            // Create sample production rate
            let production_rate = ProductionRate {
                prodction_rate_id: "RATE-001".to_string(),
                watch_model_id: "BP-2023".to_string(),
                assembly_time_per_watch: 30,
                daily_production_capacity: 40,
            };
            self.production_rate_db.put(wtxn, "RATE-001", &production_rate)?;

            // Create sample recorder point
            let recorder_point = RecorderPoint {
                recorder_point_id: "REORD-001".to_string(),
                component_name: "Premium Dial".to_string(),
                supplier_lead_time: 14,
                assumed_daily_usage: 5.2,
                lead_time_demand: 72.8,
                safety_stock: 36.4,
                reorder_point: 110,
                need_to_order: true,
            };
            self.recorder_point_db.put(wtxn, "REORD-001", &recorder_point)?;

            // Create sample watch
            let watch = Watches {
                watch_id: "WATCH-001".to_string(),
                watch_model_id: "BP-2023-001".to_string(),
                brand: "BrandX".to_string(),
                component_id: "COMP-001".to_string(),
                required_quantity: 1,
            };
            self.watches_db.put(wtxn, "WATCH-001", &watch)?;

            Ok(())
        })
    }

    // ========== INVENTORY SUMMARY ==========
    pub fn print_inventory_summary(&self) -> Result<(), heed::Error> {
        self.with_read_txn(|rtxn| {
            println!("=== PRODUCTS INVENTORY ===");
            for item in self.products_db.iter(rtxn)? {
                let (_, product) = item?;
                println!("{} (ID: {})", product.product_name, product.product_id);
                println!("  Total Available: {}", product.total_available);
                println!("  Reserved: {}", product.reserver_for_orders);
                println!(
                    "  Locations - CN: {}, Kling: {}, Wurenlos: {}",
                    product.cn,
                    product.kling,
                    product.wurenlos
                );
            }

            println!("\n=== COMPONENTS INVENTORY ===");
            for item in self.components_db.iter(rtxn)? {
                let (_, component) = item?;
                println!("{} (ID: {})", component.component_name, component.component_id);
                println!("  Total Available: {}", component.total_available);
                println!("  Ordered Surplus: {}", component.ordered_surplus);
            }

            println!("\n=== PENDING ORDERS ===");
            for item in self.orders_db.iter(rtxn)? {
                let (_, order) = item?;
                if order.order_status != "Completed" {
                    println!("Order {} - Status: {}", order.order_id, order.order_status);
                    println!("  Product: {}, Quantity: {}", order.product, order.quanity_ordered);
                }
            }

            Ok(())
        })
    }

    // ========== PRODUCTS CRUD ==========
    pub fn create_product(&self, product: &Products) -> Result<(), heed::Error> {
        let mut wtxn = self.env.write_txn()?;
        self.products_db.put(&mut wtxn, &product.product_id, product)?;
        wtxn.commit()
    }

    pub fn get_product(&self, id: &str) -> Result<Option<Products>, heed::Error> {
        let rtxn = self.env.read_txn()?;
        self.products_db.get(&rtxn, id)
    }

    pub fn update_product(&self, product: &Products) -> Result<(), heed::Error> {
        self.create_product(product)
    }

    pub fn delete_product(&self, id: &str) -> Result<bool, heed::Error> {
        let mut wtxn = self.env.write_txn()?;
        let deleted = self.products_db.delete(&mut wtxn, id)?;
        wtxn.commit()?;
        Ok(deleted)
    }

    pub fn get_all_products(&self) -> Result<Vec<Products>, heed::Error> {
        let rtxn = self.env.read_txn()?;
        self.products_db
            .iter(&rtxn)?
            .map(|res| res.map(|(_, v)| v))
            .collect()
    }

    // ========== COMPONENTS CRUD ==========
    pub fn create_component(&self, component: &Components) -> Result<(), heed::Error> {
        let mut wtxn = self.env.write_txn()?;
        self.components_db.put(&mut wtxn, &component.component_id, component)?;
        wtxn.commit()
    }

    pub fn get_component(&self, id: &str) -> Result<Option<Components>, heed::Error> {
        let rtxn = self.env.read_txn()?;
        self.components_db.get(&rtxn, id)
    }

    pub fn update_component(&self, component: &Components) -> Result<(), heed::Error> {
        self.create_component(component)
    }

    pub fn delete_component(&self, id: &str) -> Result<bool, heed::Error> {
        let mut wtxn = self.env.write_txn()?;
        let deleted = self.components_db.delete(&mut wtxn, id)?;
        wtxn.commit()?;
        Ok(deleted)
    }

    pub fn get_all_components(&self) -> Result<Vec<Components>, heed::Error> {
        let rtxn = self.env.read_txn()?;
        self.components_db
            .iter(&rtxn)?
            .map(|res| res.map(|(_, v)| v))
            .collect()
    }

    // ========== RELATIONSHIP MANAGEMENT ==========
    pub fn add_component_to_product(
        &self,
        product_id: &str,
        component_id: &str
    ) -> Result<(), heed::Error> {
        let mut wtxn = self.env.write_txn()?;

        if let Some(mut product) = self.products_db.get(&wtxn, product_id)? {
            match product.components {
                Some(ref mut components) if !components.contains(&component_id.to_string()) => {
                    components.push(component_id.to_string());
                }
                None => {
                    product.components = Some(vec![component_id.to_string()]);
                }
                _ => {}
            }

            self.products_db.put(&mut wtxn, product_id, &product)?;
        }

        wtxn.commit()
    }

    pub fn get_product_components(&self, product_id: &str) -> Result<Vec<Components>, heed::Error> {
        let rtxn = self.env.read_txn()?;

        let components = if let Some(product) = self.products_db.get(&rtxn, product_id)? {
            product.components.unwrap_or_default()
        } else {
            return Ok(Vec::new());
        };

        let mut result = Vec::new();
        for id in components {
            if let Some(component) = self.components_db.get(&rtxn, &id)? {
                result.push(component);
            }
        }
        Ok(result)
    }
    // ========== INVENTORY MOVEMENTS ==========
    pub fn record_movement(&self, movement: &Movements) -> Result<(), heed::Error> {
        let mut wtxn = self.env.write_txn()?;

        // Update source inventory
        if let Some(component_name) = &movement.component_name {
            if let Some(mut component) = self.components_db.get(&wtxn, component_name)? {
                match movement.source_location.as_str() {
                    "CN" => {
                        component.cn -= movement.quantity;
                    }
                    "Kling" => {
                        component.kling -= movement.quantity;
                    }
                    "St Jakob" => {
                        component.st_jacob -= movement.quantity;
                    }
                    "Wurenlos" => {
                        component.wurenlos -= movement.quantity;
                    }
                    "FLF" => {
                        component.flf -= movement.quantity;
                    }
                    _ => {}
                }
                self.components_db.put(&mut wtxn, component_name, &component)?;
            }
        }

        // Update destination inventory
        if let Some(component_name) = &movement.component_name {
            if let Some(mut component) = self.components_db.get(&wtxn, component_name)? {
                match movement.destination_location.as_str() {
                    "CN" => {
                        component.cn += movement.quantity;
                    }
                    "Kling" => {
                        component.kling += movement.quantity;
                    }
                    "St Jakob" => {
                        component.st_jacob += movement.quantity;
                    }
                    "Wurenlos" => {
                        component.wurenlos += movement.quantity;
                    }
                    "FLF" => {
                        component.flf += movement.quantity;
                    }
                    _ => {}
                }
                self.components_db.put(&mut wtxn, component_name, &component)?;
            }
        }

        // Record the movement
        self.movements_db.put(&mut wtxn, &movement.movement_id, movement)?;

        wtxn.commit()
    }

    pub fn get_movement(&self, id: &str) -> Result<Option<Movements>, heed::Error> {
        let rtxn = self.env.read_txn()?;
        self.movements_db.get(&rtxn, id)
    }

    pub fn get_all_movements(&self) -> Result<Vec<Movements>, heed::Error> {
        let rtxn = self.env.read_txn()?;
        self.movements_db
            .iter(&rtxn)?
            .map(|res| res.map(|(_, v)| v))
            .collect()
    }

    // ========== ORDERS MANAGEMENT ==========
    pub fn create_order(&self, order: &Orders) -> Result<(), heed::Error> {
        let mut wtxn = self.env.write_txn()?;
        self.orders_db.put(&mut wtxn, &order.order_id, order)?;
        wtxn.commit()
    }

    pub fn get_order(&self, id: &str) -> Result<Option<Orders>, heed::Error> {
        let rtxn = self.env.read_txn()?;
        self.orders_db.get(&rtxn, id)
    }

    pub fn update_order(&self, order: &Orders) -> Result<(), heed::Error> {
        self.create_order(order)
    }

    pub fn delete_order(&self, id: &str) -> Result<bool, heed::Error> {
        let mut wtxn = self.env.write_txn()?;
        let deleted = self.orders_db.delete(&mut wtxn, id)?;
        wtxn.commit()?;
        Ok(deleted)
    }

    pub fn get_all_orders(&self) -> Result<Vec<Orders>, heed::Error> {
        let rtxn = self.env.read_txn()?;
        self.orders_db
            .iter(&rtxn)?
            .map(|res| res.map(|(_, v)| v))
            .collect()
    }

    // ========== INVENTORY QUERIES ==========
    pub fn get_inventory_levels(
        &self,
        location: &str
    ) -> Result<HashMap<String, u64>, heed::Error> {
        let rtxn = self.env.read_txn()?;
        let mut levels = HashMap::new();

        for result in self.components_db.iter(&rtxn)? {
            let (_, component) = result?;
            let quantity = match location {
                "CN" => component.cn,
                "Kling" => component.kling,
                "St Jakob" => component.st_jacob,
                "Wurenlos" => component.wurenlos,
                "FLF" => component.flf,
                _ => 0,
            };
            levels.insert(component.component_name, quantity);
        }

        Ok(levels)
    }
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("cargo:rustc-link-lib=advapi32");
    let db_path = Path::new("./inventory_db");
    let inventory_db = InventoryDB::new(db_path).expect("Failed to create database");
    
    // Initialize with sample data
    inventory_db.initialize_sample_data().expect("Failed to initialize sample data");
    println!("Database initialized with sample data");

    // Print inventory summary (for debugging)
    inventory_db.print_inventory_summary().expect("Failed to print inventory summary");

    // Example: Get a product (for debugging)
    if let Some(product) = inventory_db.get_product("PROD-001").expect("Failed to get product") {
        println!("\nRetrieved product: {}", product.product_name);
    }

    // Example: Get inventory levels (for debugging)
    let cn_levels = inventory_db.get_inventory_levels("CN").expect("Failed to get inventory levels");
    println!("\nCN Inventory Levels:");
    for (component, quantity) in cn_levels {
        println!("{}: {}", component, quantity);
    }

    // Create app state with Arc-wrapped database
    let app_state = routes::AppState {
        db: Arc::new(inventory_db),
    };

    // Start HTTP server
    println!("Starting server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .wrap(actix_cors::Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600))
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(app_state.clone()))
            .configure(routes::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}