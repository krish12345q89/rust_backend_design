use serde::{ Serialize, Deserialize };
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Products {
    pub product_name: String,
    pub product_id: String,
    pub components: Option<Vec<String>>,
    pub cn: u64,
    pub kling: u64,
    pub st_jacob: u64,
    pub wurenlos: u64,
    pub wurenlos_sold: u64,
    pub flf: u64,
    pub in_transit: u64,
    pub total_available: u64,
    pub reserver_for_orders: u64,
    pub waste: u64,
    pub customer: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Components {
    pub product_id: String,
    pub product_name: String,
    pub component_name: String,
    pub component_id: String,
    pub cn: u64,
    pub kling: u64,
    pub st_jacob: u64,
    pub wurenlos: u64,
    pub wurenlos_sold: u64,
    pub flf: u64,
    pub in_transit: u64,
    pub total_available: u64,
    pub ordered_surplus: f64,
    pub reserver_for_orders: u64,
    pub waste: u64,
    pub customer: u64,
    pub assembly_line: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Movements {
    pub movement_id: String,
    pub transaction_id: String,
    pub date: NaiveDate,
    pub movement_type: String,
    pub component_name: Option<String>,
    pub product_name: Option<String>,
    pub source_location: String,
    pub destination_location: String,
    pub quantity: u64,
    pub notes: Option<String>,
    pub status: String,
    pub supplier_order_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuppliersOrders {
    pub supplier_id: String,
    pub component_name: String,
    pub procurement_id: String,
    pub order_id: String,
    pub total_components_required: u64,
    pub components_roundof: u64,
    pub status: String,
    pub order_date: NaiveDate,
    pub expected_delivery_date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Orders {
    pub order_id: String,
    pub procurements: Option<Vec<String>>,
    pub supplier_orders: Option<Vec<String>>,
    pub quanity_ordered: u64,
    pub product_id: String,
    pub product: String,
    pub quantity_required: u64,
    pub expected_delivery_date: NaiveDate,
    pub production_start_date: NaiveDate,
    pub expected_ship_date: NaiveDate,
    pub recid: String,
    pub order_status: String,
    pub total_components_booked: u64,
    pub components_notes: Option<String>,
    pub components_required: u64,
    pub total_gap_components: Option<Vec<u64>>,
    pub components: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Procurement {
    pub procurement_id: String,
    pub order_id: String,
    pub components: Option<Vec<String>>,
    pub quantity: u64,
    pub status: String,
    pub product: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Procurements {
    pub procurement_id: String,
    pub order_id: String,
    pub procurements: Vec<Procurement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssemblyTimeLine {
    pub assembly_id: String,
    pub order: String,
    pub product: String,
    pub movements: Vec<String>,
    pub components_required: u64,
    pub total_components_booked: u64,
    pub components: Vec<String>,
    pub total_gap_components: Option<Vec<u64>>,
    pub assembly_location: String,
    pub components_received_date: NaiveDate,
    pub assembly_start_date: NaiveDate,
    pub assembly_end_date: NaiveDate,
    pub assembly_status: String,
    pub total_duration: u64,
    pub assembly_notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductionRate {
    pub prodction_rate_id: String,
    pub watch_model_id: String,
    pub assembly_time_per_watch: u64,
    pub daily_production_capacity: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecorderPoint {
    pub recorder_point_id: String,
    pub component_name: String,
    pub supplier_lead_time: u64,
    pub assumed_daily_usage: f64,
    pub lead_time_demand: f64,
    pub safety_stock: f64,
    pub reorder_point: u64,
    pub need_to_order: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Watches {
    pub watch_id: String,
    pub watch_model_id: String,
    pub brand: String,
    pub component_id: String,
    pub required_quantity: u64,
}
