extern crate juniper;

use juniper::FieldResult;

use super::{
    DiskInformation, LoadAverage, System, MemoryInformation, ByteUnit, CycleUnit
};


graphql_enum!(ByteUnit {
    ByteUnit::KB => "KB",
    ByteUnit::MB => "MB",
    ByteUnit::GB => "GB",
    ByteUnit::TB => "TB",
});

graphql_enum!(CycleUnit {
    CycleUnit::MHz => "MHz",
    CycleUnit::GHz => "GHz"
});

graphql_object!(DiskInformation: System as "DiskInformation" |&self| {
    description: "Get disk information"

    field total() -> FieldResult<&String> as "Total disk space" {
        Ok(&self.total)
    }

    field free() -> FieldResult<&String> as "Available free disk space" {
        Ok(&self.free)
    }
});

graphql_object!(LoadAverage: System as "LoadAverage" |&self| {
    description: "CPU load averages"

    field one() -> FieldResult<&String> as "One minute average" {
        Ok(&self.one)
    }

    field five() -> FieldResult<&String> as "Five minute average" {
        Ok(&self.five)
    }

    field fifteen() -> FieldResult<&String> as "Fifteen minute average" {
        Ok(&self.fifteen)
    }
});

graphql_object!(MemoryInformation: System as "MemoryInformation" |&self| {
    description: "Get memory information"

    field total() -> FieldResult<&String> as "Total memory" {
        Ok(&self.total)
    }

    field free() -> FieldResult<&String> as "Free memory" {
        Ok(&self.free)
    }

    field available() -> FieldResult<&String> as "Available memory" {
        Ok(&self.available)
    }


    field buffers() -> FieldResult<&String> as "Buffers" {
        Ok(&self.buffers)
    }


    field cached() -> FieldResult<&String> as "Cached memory" {
        Ok(&self.cached)
    }


    field swap_total() -> FieldResult<&String> as "Swap total" {
        Ok(&self.swap_total)
    }

    field swap_free() -> FieldResult<&String> as "Swap free" {
        Ok(&self.swap_free)
    }
});

graphql_object!(System: System as "Query" |&self| {
    description: "The root query object of the schema"

    field disk_info(
        &executor,
        unit: Option<ByteUnit> as "Byte size. Defaults to KB."
    ) -> Option<DiskInformation> {
        Some(self.get_disk_info(unit))
    }

    field load_average(&executor) -> Option<LoadAverage> {
        Some(self.get_load_average())
    }

    field memory_info(
        &executor,
        unit: Option<ByteUnit> as "Byte size. Defaults to KB."
    ) -> Option<MemoryInformation> {
        Some(self.get_memory_information(unit))
    }

    field os_type(&executor) -> &String as "Operating system type" {
        &self.os_type
    }

    field os_release(&executor) -> &String as "Operating system release version" {
        &self.os_release
    }

    field hostname(&executor) -> &String as "Hostname" {
        &self.hostname
    }

    field cpu_count(&executor) -> &String as "CPU quantity." {
        &self.cpu_num
    }

    field cpu_speed(
        &executor,
        unit: Option<CycleUnit> as "Hertz unit size. Defaults to MHz."
    ) -> String as "CPU speed. Such as 2500 MHz." {
        self.get_cpu_speed(unit)
    }

    field proc_count(&executor) -> String as "Current quantity of processes." {
        self.get_proc_total()
    }

    field boot_time(&executor) -> String as "System boot time. Seconds system has been up and seconds that the machine has spend idle." {
        self.get_boot_time()
    }
});
