use crate::utils::TaskPerfAggregator;
use cardano_multiplatform_lib::genesis::byron::config::GenesisData;
use entity::{prelude::*, sea_orm::DatabaseTransaction};
use pallas::ledger::primitives::{alonzo, byron};
use shred::DispatcherBuilder;
use std::sync::{Arc, Mutex};

pub type BlockInfo<'a, BlockType> = (
    &'a str, // cbor. Empty for genesis
    &'a BlockType,
    &'a BlockModel,
);

pub trait DatabaseTaskMeta<'a, BlockType> {
    const TASK_NAME: &'static str;
    const DEPENDENCIES: &'static [&'static str];

    fn new(
        db_tx: &'a DatabaseTransaction,
        block: BlockInfo<'a, BlockType>,
        handle: &'a tokio::runtime::Handle,
        perf_aggregator: Arc<Mutex<TaskPerfAggregator>>,
    ) -> Self;
}

pub trait TaskBuilder<'a, BlockType> {
    fn get_name(&self) -> &'static str;
    fn get_dependencies(&self) -> &'static [&'static str];

    fn add_task<'c>(
        &self,
        dispatcher_builder: &mut DispatcherBuilder<'a, 'c>,
        db_tx: &'a DatabaseTransaction,
        block: BlockInfo<'a, BlockType>,
        handle: &'a tokio::runtime::Handle,
        perf_aggregator: Arc<Mutex<TaskPerfAggregator>>,
        properties: &ini::Properties,
    );
}

#[derive(Copy, Clone)]
pub enum TaskRegistryEntry {
    Genesis(GenesisTaskRegistryEntry),
    Byron(ByronTaskRegistryEntry),
    Multiera(MultieraTaskRegistryEntry),
}

#[derive(Copy, Clone)]
pub struct GenesisTaskRegistryEntry {
    pub builder: &'static (dyn for<'a> TaskBuilder<'a, GenesisData> + Sync),
}

#[derive(Copy, Clone)]
pub struct ByronTaskRegistryEntry {
    pub builder: &'static (dyn for<'a> TaskBuilder<'a, byron::Block> + Sync),
}

#[derive(Copy, Clone)]
pub struct MultieraTaskRegistryEntry {
    pub builder: &'static (dyn for<'a> TaskBuilder<'a, alonzo::Block> + Sync),
}

inventory::collect!(TaskRegistryEntry);
