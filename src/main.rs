use cursive::views::TextView;

fn main() {
    let mut sis = cursive::default();

    sis.add_global_callback('q', |sis| sis.quit());

    sis.add_layer(TextView::new("Heckin', yeah, cool."));

    sis.run();
}
