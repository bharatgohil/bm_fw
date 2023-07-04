
pub mod cpsw_ethdev;
pub trait EthDev {
    fn init(&self);
    fn dev_start(&self);
    fn dev_stop(&self);
    fn stats_get(&self);
    fn rx_queue_setup(&self);
    fn tx_queue_setup(&self);
}

