use crate::content::UsageMap;
use cursive::utils::Counter;
use cursive::views::{Dialog, TextView};
use cursive::{CbSink, CursiveRunnable};
use std::borrow::BorrowMut;
use sysinfo::{System, SystemExt};

pub mod content;
pub mod structure;

pub fn create_cursive_runnable(input_duration: u64) -> CursiveRunnable {
    let mut siv = cursive::default();

    let mut sys = System::new_all();
    sys.refresh_all();
    let n_cpus = sys.cpus().len();

    let (counters, full_layout) = structure::create_interface(n_cpus);

    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(Dialog::around(full_layout));

    let cb_sink = siv.cb_sink().clone();
    let duration = std::time::Duration::from_millis(input_duration);

    std::thread::spawn(move || {
        let mut usage_store = UsageMap::new();
        loop {
            usage_store = event_loop(sys.borrow_mut(), &counters, cb_sink.clone(), usage_store);
            std::thread::sleep(duration);
        }
    });
    siv
}

fn event_loop(
    sys: &mut System,
    counters: &[Counter],
    cb_sink: CbSink,
    usage_store: UsageMap,
) -> UsageMap {
    sys.refresh_all();
    let (name_string, usage_string, cpu_usages, usage_store) =
        content::get_updated_usage(sys, usage_store);

    for (counter, value) in counters.iter().zip(&cpu_usages) {
        counter.set(*value as usize);
    }

    cb_sink
        .send(Box::new(move |s| {
            s.call_on_name("names", |v: &mut TextView| v.set_content(name_string));
            s.call_on_name("numbers", |v: &mut TextView| v.set_content(usage_string));
        }))
        .unwrap();
    usage_store
}
