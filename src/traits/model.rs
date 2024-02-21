pub trait ModelTrait {
    fn set_id(&mut self, id: String);
    fn set_created_at(&mut self, created_at: u64);
    fn set_updated_at(&mut self, updated_at: u64);
}
