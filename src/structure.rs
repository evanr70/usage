use cursive::align;
use cursive::traits::Nameable;
use cursive::utils::Counter;
use cursive::views::{Dialog, LinearLayout, PaddedView, ProgressBar, TextView};

pub fn create_interface(n_cpus: usize) -> (Vec<Counter>, LinearLayout) {
    let counters: Vec<_> = (0..n_cpus).map(|_| Counter::new(0)).collect();

    let mut full_layout = LinearLayout::vertical();
    let title_view = TextView::new("evanr70/usage").h_align(align::HAlign::Center);
    let mut user_layout = LinearLayout::horizontal();
    let name_view = TextView::new("Starting").with_name("names");
    let user_usage_view = PaddedView::lrtb(
        2,
        0,
        0,
        0,
        TextView::new("Calculating")
            .h_align(align::HAlign::Right)
            .with_name("numbers"),
    );
    let mut counter_view = LinearLayout::vertical();

    user_layout.add_child(name_view);
    user_layout.add_child(user_usage_view);

    full_layout.add_child(title_view);
    full_layout.add_child(Dialog::around(user_layout));

    for c in &counters {
        counter_view.add_child(ProgressBar::new().max(100).with_value(c.clone()));
    }

    full_layout.add_child(Dialog::around(counter_view));
    (counters, full_layout)
}
