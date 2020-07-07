use argh::FromArgs;
use serde::Deserialize;

#[derive(FromArgs)]
/// Run sample code
#[argh(subcommand, name = "sample")]
pub struct Sample {}

impl Sample {
    pub fn run(self) {
        self.read_records();
    }

    fn read_records(&self) {
        #[derive(Deserialize)]
        struct Record {
            #[allow(unused)]
            city: String,
            #[allow(unused)]
            state: String,
        }

        let f = std::fs::File::open("cities.json").unwrap();
        crate::ALLOCATOR.set_active(true);
        let records: Vec<Record> = serde_json::from_reader(f).unwrap();
        crate::ALLOCATOR.set_active(false);
        println!("Read {} records", records.len());
    }
}
