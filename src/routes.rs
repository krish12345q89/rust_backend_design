use actix_web::{
    web,
    HttpResponse,
    Responder,
    Result,
    get,
    post,
    put,
    delete,
};
use serde::{ Serialize, Deserialize };
use std::collections::HashMap;
use std::sync::Arc;

use crate::r#struct::{AssemblyTimeLine, Components, Movements, Orders, Procurements, ProductionRate, Products, RecorderPoint, SuppliersOrders, Watches};
use crate::InventoryDB;

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: Option<String>,
}

impl<T> ApiResponse<T> {
    fn success(data: T) -> ApiResponse<T> {
        ApiResponse {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    fn error(message: &str) -> ApiResponse<String> {
        ApiResponse {
            success: false,
            data: None,
            message: Some(message.to_string()),
        }
    }
}

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) db: Arc<InventoryDB>,
}

#[get("/api/products")]
async fn get_all_products(data: web::Data<AppState>) -> Result<impl Responder> {
    match data.db.get_all_products() {
        Ok(products) => Ok(HttpResponse::Ok().json(ApiResponse::<Vec<Products>>::success(products))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[get("/api/products/{id}")]
async fn get_product(data: web::Data<AppState>, path: web::Path<String>) -> Result<impl Responder> {
    let id = path.into_inner();
    match data.db.get_product(&id) {
        Ok(Some(product)) => Ok(HttpResponse::Ok().json(ApiResponse::<Products>::success(product))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Product not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[post("/api/products")]
async fn create_product(
    data: web::Data<AppState>,
    product: web::Json<Products>
) -> Result<impl Responder> {
    match data.db.create_product(&product.into_inner()) {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::<&str>::success("Product created"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[put("/api/products/{id}")]
async fn update_product(
    data: web::Data<AppState>,
    path: web::Path<String>,
    product: web::Json<Products>
) -> Result<impl Responder> {
    let id = path.into_inner();
    let mut product = product.into_inner();
    product.product_id = id;

    match data.db.update_product(&product) {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::<&str>::success("Product updated"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[delete("/api/products/{id}")]
async fn delete_product(
    data: web::Data<AppState>,
    path: web::Path<String>
) -> Result<impl Responder> {
    let id = path.into_inner();
    match data.db.delete_product(&id) {
        Ok(true) => Ok(HttpResponse::Ok().json(ApiResponse::<&str>::success("Product deleted"))),
        Ok(false) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Product not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

// ========== COMPONENTS API ==========

#[get("/api/components")]
async fn get_all_components(data: web::Data<AppState>) -> Result<impl Responder> {
    match data.db.get_all_components() {
        Ok(components) => Ok(HttpResponse::Ok().json(ApiResponse::<Vec<Components>>::success(components))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[get("/api/components/{id}")]
async fn get_component(
    data: web::Data<AppState>,
    path: web::Path<String>
) -> Result<impl Responder> {
    let id = path.into_inner();
    match data.db.get_component(&id) {
        Ok(Some(component)) => Ok(HttpResponse::Ok().json(ApiResponse::<Components>::success(component))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Component not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[post("/api/components")]
async fn create_component(
    data: web::Data<AppState>,
    component: web::Json<Components>
) -> Result<impl Responder> {
    match data.db.create_component(&component.into_inner()) {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::<&str>::success("Component created"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[put("/api/components/{id}")]
async fn update_component(
    data: web::Data<AppState>,
    path: web::Path<String>,
    component: web::Json<Components>
) -> Result<impl Responder> {
    let id = path.into_inner();
    let mut component = component.into_inner();
    component.component_id = id;

    match data.db.update_component(&component) {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::<&str>::success("Component updated"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[delete("/api/components/{id}")]
async fn delete_component(
    data: web::Data<AppState>,
    path: web::Path<String>
) -> Result<impl Responder> {
    let id = path.into_inner();
    match data.db.delete_component(&id) {
        Ok(true) => Ok(HttpResponse::Ok().json(ApiResponse::<&str>::success("Component deleted"))),
        Ok(false) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Component not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

// ========== MOVEMENTS API ==========

#[get("/api/movements")]
async fn get_all_movements(data: web::Data<AppState>) -> Result<impl Responder> {
    match data.db.get_all_movements() {
        Ok(movements) => Ok(HttpResponse::Ok().json(ApiResponse::<Vec<Movements>>::success(movements))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[get("/api/movements/{id}")]
async fn get_movement(
    data: web::Data<AppState>,
    path: web::Path<String>
) -> Result<impl Responder> {
    let id = path.into_inner();
    match data.db.get_movement(&id) {
        Ok(Some(movement)) => Ok(HttpResponse::Ok().json(ApiResponse::<Movements>::success(movement))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Movement not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[post("/api/movements")]
async fn record_movement(
    data: web::Data<AppState>,
    movement: web::Json<Movements>
) -> Result<impl Responder> {
    match data.db.record_movement(&movement.into_inner()) {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::<&str>::success("Movement recorded"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

// ========== ORDERS API ==========

#[get("/api/orders")]
async fn get_all_orders(data: web::Data<AppState>) -> Result<impl Responder> {
    match data.db.get_all_orders() {
        Ok(orders) => Ok(HttpResponse::Ok().json(ApiResponse::<Vec<Orders>>::success(orders))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[get("/api/orders/{id}")]
async fn get_order(data: web::Data<AppState>, path: web::Path<String>) -> Result<impl Responder> {
    let id = path.into_inner();
    match data.db.get_order(&id) {
        Ok(Some(order)) => Ok(HttpResponse::Ok().json(ApiResponse::<Orders>::success(order))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Order not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[post("/api/orders")]
async fn create_order(
    data: web::Data<AppState>,
    order: web::Json<Orders>
) -> Result<impl Responder> {
    match data.db.create_order(&order.into_inner()) {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::<&str>::success("Order created"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[put("/api/orders/{id}")]
async fn update_order(
    data: web::Data<AppState>,
    path: web::Path<String>,
    order: web::Json<Orders>
) -> Result<impl Responder> {
    let id = path.into_inner();
    let mut order = order.into_inner();
    order.order_id = id;

    match data.db.update_order(&order) {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::<&str>::success("Order updated"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[delete("/api/orders/{id}")]
async fn delete_order(
    data: web::Data<AppState>,
    path: web::Path<String>
) -> Result<impl Responder> {
    let id = path.into_inner();
    match data.db.delete_order(&id) {
        Ok(true) => Ok(HttpResponse::Ok().json(ApiResponse::<&str>::success("Order deleted"))),
        Ok(false) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Order not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

// ========== INVENTORY API ==========

#[get("/api/inventory/{location}")]
async fn get_inventory_levels(
    data: web::Data<AppState>,
    path: web::Path<String>
) -> Result<impl Responder> {
    let location = path.into_inner();
    match data.db.get_inventory_levels(&location) {
        Ok(levels) => Ok(HttpResponse::Ok().json(ApiResponse::<HashMap<String, u64>>::success(levels))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[get("/api/products/{id}/components")]
async fn get_product_components(
    data: web::Data<AppState>,
    path: web::Path<String>
) -> Result<impl Responder> {
    let id = path.into_inner();
    match data.db.get_product_components(&id) {
        Ok(components) => Ok(HttpResponse::Ok().json(ApiResponse::<Vec<Components>>::success(components))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[post("/api/products/{product_id}/components/{component_id}")]
async fn add_component_to_product(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>
) -> Result<impl Responder> {
    let (product_id, component_id) = path.into_inner();
    match data.db.add_component_to_product(&product_id, &component_id) {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::<&str>::success("Component added to product"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}
// Add these to your existing routes.rs file

// ========== SUPPLIER ORDERS API ==========

#[get("/api/supplier-orders")]
async fn get_all_supplier_orders(data: web::Data<AppState>) -> Result<impl Responder> {
    match data.db.with_read_txn(|rtxn| {
        data.db.suppliers_orders_db
            .iter(&rtxn)?
            .map(|res| res.map(|(_, v)| v))
            .collect::<heed::Result<Vec<SuppliersOrders>>>()
    }) {
        Ok(orders) => Ok(HttpResponse::Ok().json(ApiResponse::<Vec<SuppliersOrders>>::success(orders))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[get("/api/supplier-orders/{id}")]
async fn get_supplier_order(
    data: web::Data<AppState>,
    path: web::Path<String>
) -> Result<impl Responder> {
    let id = path.into_inner();
    match data.db.with_read_txn(|rtxn| data.db.suppliers_orders_db.get(&rtxn, &id)) {
        Ok(Some(order)) => Ok(HttpResponse::Ok().json(ApiResponse::<SuppliersOrders>::success(order))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Supplier order not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[post("/api/supplier-orders")]
async fn create_supplier_order(
    data: web::Data<AppState>,
    order: web::Json<SuppliersOrders>
) -> Result<impl Responder> {
    let order = order.into_inner();
    match data.db.with_write_txn(|wtxn| {
        data.db.suppliers_orders_db.put(wtxn, &order.order_id, &order)
    }) {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::<&str>::success("Supplier order created"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[put("/api/supplier-orders/{id}")]
async fn update_supplier_order(
    data: web::Data<AppState>,
    path: web::Path<String>,
    order: web::Json<SuppliersOrders>
) -> Result<impl Responder> {
    let id = path.into_inner();
    let mut order = order.into_inner();
    order.order_id = id;

    match data.db.with_write_txn(|wtxn| {
        data.db.suppliers_orders_db.put(wtxn, &order.order_id, &order)
    }) {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::<&str>::success("Supplier order updated"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[delete("/api/supplier-orders/{id}")]
async fn delete_supplier_order(
    data: web::Data<AppState>,
    path: web::Path<String>
) -> Result<impl Responder> {
    let id = path.into_inner();
    match data.db.with_write_txn(|wtxn| {
        data.db.suppliers_orders_db.delete(wtxn, &id)
    }) {
        Ok(true) => Ok(HttpResponse::Ok().json(ApiResponse::<&str>::success("Supplier order deleted"))),
        Ok(false) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Supplier order not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

// ========== PROCUREMENTS API ==========

#[get("/api/procurements")]
async fn get_all_procurements(data: web::Data<AppState>) -> Result<impl Responder> {
    match data.db.with_read_txn(|rtxn| {
        data.db.procurements_db
            .iter(&rtxn)?
            .map(|res| res.map(|(_, v)| v))
            .collect::<heed::Result<Vec<Procurements>>>()
    }) {
        Ok(procurements) => Ok(HttpResponse::Ok().json(ApiResponse::<Vec<Procurements>>::success(procurements))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[get("/api/procurements/{id}")]
async fn get_procurement(
    data: web::Data<AppState>,
    path: web::Path<String>
) -> Result<impl Responder> {
    let id = path.into_inner();
    match data.db.with_read_txn(|rtxn| data.db.procurements_db.get(&rtxn, &id)) {
        Ok(Some(procurement)) => Ok(HttpResponse::Ok().json(ApiResponse::<Procurements>::success(procurement))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Procurement not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[post("/api/procurements")]
async fn create_procurement(
    data: web::Data<AppState>,
    procurement: web::Json<Procurements>
) -> Result<impl Responder> {
    let procurement = procurement.into_inner();
    match data.db.with_write_txn(|wtxn| {
        data.db.procurements_db.put(wtxn, &procurement.procurement_id, &procurement)
    }) {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::<&str>::success("Procurement created"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[put("/api/procurements/{id}")]
async fn update_procurement(
    data: web::Data<AppState>,
    path: web::Path<String>,
    procurement: web::Json<Procurements>
) -> Result<impl Responder> {
    let id = path.into_inner();
    let mut procurement = procurement.into_inner();
    procurement.procurement_id = id;

    match data.db.with_write_txn(|wtxn| {
        data.db.procurements_db.put(wtxn, &procurement.procurement_id, &procurement)
    }) {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::<&str>::success("Procurement updated"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[delete("/api/procurements/{id}")]
async fn delete_procurement(
    data: web::Data<AppState>,
    path: web::Path<String>
) -> Result<impl Responder> {
    let id = path.into_inner();
    match data.db.with_write_txn(|wtxn| {
        data.db.procurements_db.delete(wtxn, &id)
    }) {
        Ok(true) => Ok(HttpResponse::Ok().json(ApiResponse::<&str>::success("Procurement deleted"))),
        Ok(false) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Procurement not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

// ========== ASSEMBLY TIMELINE API ==========

#[get("/api/assembly-timeline")]
async fn get_all_assembly_timelines(data: web::Data<AppState>) -> Result<impl Responder> {
    match data.db.with_read_txn(|rtxn| {
        data.db.assembly_timeline_db
            .iter(&rtxn)?
            .map(|res| res.map(|(_, v)| v))
            .collect::<heed::Result<Vec<AssemblyTimeLine>>>()
    }) {
        Ok(timelines) => Ok(HttpResponse::Ok().json(ApiResponse::<Vec<AssemblyTimeLine>>::success(timelines))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[get("/api/assembly-timeline/{id}")]
async fn get_assembly_timeline(
    data: web::Data<AppState>,
    path: web::Path<String>
) -> Result<impl Responder> {
    let id = path.into_inner();
    match data.db.with_read_txn(|rtxn| data.db.assembly_timeline_db.get(&rtxn, &id)) {
        Ok(Some(timeline)) => Ok(HttpResponse::Ok().json(ApiResponse::<AssemblyTimeLine>::success(timeline))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Assembly timeline not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[post("/api/assembly-timeline")]
async fn create_assembly_timeline(
    data: web::Data<AppState>,
    timeline: web::Json<AssemblyTimeLine>
) -> Result<impl Responder> {
    let timeline = timeline.into_inner();
    match data.db.with_write_txn(|wtxn| {
        data.db.assembly_timeline_db.put(wtxn, &timeline.assembly_id, &timeline)
    }) {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::<&str>::success("Assembly timeline created"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[put("/api/assembly-timeline/{id}")]
async fn update_assembly_timeline(
    data: web::Data<AppState>,
    path: web::Path<String>,
    timeline: web::Json<AssemblyTimeLine>
) -> Result<impl Responder> {
    let id = path.into_inner();
    let mut timeline = timeline.into_inner();
    timeline.assembly_id = id;

    match data.db.with_write_txn(|wtxn| {
        data.db.assembly_timeline_db.put(wtxn, &timeline.assembly_id, &timeline)
    }) {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::<&str>::success("Assembly timeline updated"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[delete("/api/assembly-timeline/{id}")]
async fn delete_assembly_timeline(
    data: web::Data<AppState>,
    path: web::Path<String>
) -> Result<impl Responder> {
    let id = path.into_inner();
    match data.db.with_write_txn(|wtxn| {
        data.db.assembly_timeline_db.delete(wtxn, &id)
    }) {
        Ok(true) => Ok(HttpResponse::Ok().json(ApiResponse::<&str>::success("Assembly timeline deleted"))),
        Ok(false) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Assembly timeline not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

// ========== PRODUCTION RATE API ==========

#[get("/api/production-rates")]
async fn get_all_production_rates(data: web::Data<AppState>) -> Result<impl Responder> {
    match data.db.with_read_txn(|rtxn| {
        data.db.production_rate_db
            .iter(&rtxn)?
            .map(|res| res.map(|(_, v)| v))
            .collect::<heed::Result<Vec<ProductionRate>>>()
    }) {
        Ok(rates) => Ok(HttpResponse::Ok().json(ApiResponse::<Vec<ProductionRate>>::success(rates))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[get("/api/production-rates/{id}")]
async fn get_production_rate(
    data: web::Data<AppState>,
    path: web::Path<String>
) -> Result<impl Responder> {
    let id = path.into_inner();
    match data.db.with_read_txn(|rtxn| data.db.production_rate_db.get(&rtxn, &id)) {
        Ok(Some(rate)) => Ok(HttpResponse::Ok().json(ApiResponse::<ProductionRate>::success(rate))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Production rate not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[post("/api/production-rates")]
async fn create_production_rate(
    data: web::Data<AppState>,
    rate: web::Json<ProductionRate>
) -> Result<impl Responder> {
    let rate = rate.into_inner();
    match data.db.with_write_txn(|wtxn| {
        data.db.production_rate_db.put(wtxn, &rate.prodction_rate_id, &rate)
    }) {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::<&str>::success("Production rate created"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[put("/api/production-rates/{id}")]
async fn update_production_rate(
    data: web::Data<AppState>,
    path: web::Path<String>,
    rate: web::Json<ProductionRate>
) -> Result<impl Responder> {
    let id = path.into_inner();
    let mut rate = rate.into_inner();
    rate.prodction_rate_id = id;

    match data.db.with_write_txn(|wtxn| {
        data.db.production_rate_db.put(wtxn, &rate.prodction_rate_id, &rate)
    }) {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::<&str>::success("Production rate updated"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[delete("/api/production-rates/{id}")]
async fn delete_production_rate(
    data: web::Data<AppState>,
    path: web::Path<String>
) -> Result<impl Responder> {
    let id = path.into_inner();
    match data.db.with_write_txn(|wtxn| {
        data.db.production_rate_db.delete(wtxn, &id)
    }) {
        Ok(true) => Ok(HttpResponse::Ok().json(ApiResponse::<&str>::success("Production rate deleted"))),
        Ok(false) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Production rate not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

// ========== REORDER POINTS API ==========

#[get("/api/reorder-points")]
async fn get_all_reorder_points(data: web::Data<AppState>) -> Result<impl Responder> {
    match data.db.with_read_txn(|rtxn| {
        data.db.recorder_point_db
            .iter(&rtxn)?
            .map(|res| res.map(|(_, v)| v))
            .collect::<heed::Result<Vec<RecorderPoint>>>()
    }) {
        Ok(points) => Ok(HttpResponse::Ok().json(ApiResponse::<Vec<RecorderPoint>>::success(points))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[get("/api/reorder-points/{id}")]
async fn get_reorder_point(
    data: web::Data<AppState>,
    path: web::Path<String>
) -> Result<impl Responder> {
    let id = path.into_inner();
    match data.db.with_read_txn(|rtxn| data.db.recorder_point_db.get(&rtxn, &id)) {
        Ok(Some(point)) => Ok(HttpResponse::Ok().json(ApiResponse::<RecorderPoint>::success(point))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Reorder point not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[post("/api/reorder-points")]
async fn create_reorder_point(
    data: web::Data<AppState>,
    point: web::Json<RecorderPoint>
) -> Result<impl Responder> {
    let point = point.into_inner();
    match data.db.with_write_txn(|wtxn| {
        data.db.recorder_point_db.put(wtxn, &point.recorder_point_id, &point)
    }) {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::<&str>::success("Reorder point created"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[put("/api/reorder-points/{id}")]
async fn update_reorder_point(
    data: web::Data<AppState>,
    path: web::Path<String>,
    point: web::Json<RecorderPoint>
) -> Result<impl Responder> {
    let id = path.into_inner();
    let mut point = point.into_inner();
    point.recorder_point_id = id;

    match data.db.with_write_txn(|wtxn| {
        data.db.recorder_point_db.put(wtxn, &point.recorder_point_id, &point)
    }) {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::<&str>::success("Reorder point updated"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[delete("/api/reorder-points/{id}")]
async fn delete_reorder_point(
    data: web::Data<AppState>,
    path: web::Path<String>
) -> Result<impl Responder> {
    let id = path.into_inner();
    match data.db.with_write_txn(|wtxn| {
        data.db.recorder_point_db.delete(wtxn, &id)
    }) {
        Ok(true) => Ok(HttpResponse::Ok().json(ApiResponse::<&str>::success("Reorder point deleted"))),
        Ok(false) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Reorder point not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

// ========== WATCHES API ==========

#[get("/api/watches")]
async fn get_all_watches(data: web::Data<AppState>) -> Result<impl Responder> {
    match data.db.with_read_txn(|rtxn| {
        data.db.watches_db
            .iter(&rtxn)?
            .map(|res| res.map(|(_, v)| v))
            .collect::<heed::Result<Vec<Watches>>>()
    }) {
        Ok(watches) => Ok(HttpResponse::Ok().json(ApiResponse::<Vec<Watches>>::success(watches))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[get("/api/watches/{id}")]
async fn get_watch(
    data: web::Data<AppState>,
    path: web::Path<String>
) -> Result<impl Responder> {
    let id = path.into_inner();
    match data.db.with_read_txn(|rtxn| data.db.watches_db.get(&rtxn, &id)) {
        Ok(Some(watch)) => Ok(HttpResponse::Ok().json(ApiResponse::<Watches>::success(watch))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Watch not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[post("/api/watches")]
async fn create_watch(
    data: web::Data<AppState>,
    watch: web::Json<Watches>
) -> Result<impl Responder> {
    let watch = watch.into_inner();
    match data.db.with_write_txn(|wtxn| {
        data.db.watches_db.put(wtxn, &watch.watch_id, &watch)
    }) {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::<&str>::success("Watch created"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[put("/api/watches/{id}")]
async fn update_watch(
    data: web::Data<AppState>,
    path: web::Path<String>,
    watch: web::Json<Watches>
) -> Result<impl Responder> {
    let id = path.into_inner();
    let mut watch = watch.into_inner();
    watch.watch_id = id;

    match data.db.with_write_txn(|wtxn| {
        data.db.watches_db.put(wtxn, &watch.watch_id, &watch)
    }) {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::<&str>::success("Watch updated"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

#[delete("/api/watches/{id}")]
async fn delete_watch(
    data: web::Data<AppState>,
    path: web::Path<String>
) -> Result<impl Responder> {
    let id = path.into_inner();
    match data.db.with_write_txn(|wtxn| {
        data.db.watches_db.delete(wtxn, &id)
    }) {
        Ok(true) => Ok(HttpResponse::Ok().json(ApiResponse::<&str>::success("Watch deleted"))),
        Ok(false) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Watch not found"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<String>::error(&e.to_string()))),
    }
}

// Update the init_routes function to include all new routes
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_products)
        .service(get_product)
        .service(create_product)
        .service(update_product)
        .service(delete_product)
        .service(get_all_components)
        .service(get_component)
        .service(create_component)
        .service(update_component)
        .service(delete_component)
        .service(get_all_movements)
        .service(get_movement)
        .service(record_movement)
        .service(get_all_orders)
        .service(get_order)
        .service(create_order)
        .service(update_order)
        .service(delete_order)
        .service(get_all_supplier_orders)
        .service(get_supplier_order)
        .service(create_supplier_order)
        .service(update_supplier_order)
        .service(delete_supplier_order)
        .service(get_all_procurements)
        .service(get_procurement)
        .service(create_procurement)
        .service(update_procurement)
        .service(delete_procurement)
        .service(get_all_assembly_timelines)
        .service(get_assembly_timeline)
        .service(create_assembly_timeline)
        .service(update_assembly_timeline)
        .service(delete_assembly_timeline)
        .service(get_all_production_rates)
        .service(get_production_rate)
        .service(create_production_rate)
        .service(update_production_rate)
        .service(delete_production_rate)
        .service(get_all_reorder_points)
        .service(get_reorder_point)
        .service(create_reorder_point)
        .service(update_reorder_point)
        .service(delete_reorder_point)
        .service(get_all_watches)
        .service(get_watch)
        .service(create_watch)
        .service(update_watch)
        .service(delete_watch)
        .service(get_inventory_levels)
        .service(get_product_components)
        .service(add_component_to_product);
}