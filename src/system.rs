use sysinfo::System;

use crate::{
    args::{SysMemArgs, SystemCommands},
    errors::Error,
};

pub fn parse(cmd: &SystemCommands) -> Result<(), Error> {
    let mut data = Data::new();

    match cmd {
        SystemCommands::Cpu => data.cpu(),
        SystemCommands::Cpus => todo!(),
        SystemCommands::Memory(args) => data.memory(args),
        SystemCommands::Disk => todo!(),
    }
}

struct Data {
    sys: System,
}

impl Data {
    fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        Data { sys }
    }

    fn cpu(&mut self) -> Result<(), Error> {
        self.sys.refresh_cpu();

        let cpu = self.sys.global_cpu_info();
        let cpu = cpu.cpu_usage();
        println!("{:#?}", cpu);

        Ok(())
    }

    fn memory(&mut self, args: &SysMemArgs) -> Result<(), Error> {
        let mut count = self.sys.total_memory();

        if args.usage {
            count = self.sys.used_memory();
        }

        println!("{}", convert_bytes_to_unit(count));

        Ok(())
    }
}

fn convert_bytes_to_unit(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}
