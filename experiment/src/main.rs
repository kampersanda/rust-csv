use csv_core::{ReadFieldResult, Reader};

fn main() {
    let data = "foo,bar,baz\na,b,c\nxxx,yyy,zzz\n";
    // let data = "foo,bar,baz\r\na,b,c\r\nxxx,yyy,zzz\r\n"; # Failed!

    let mut rdr = Reader::new();
    let mut bytes = data.as_bytes();
    let mut count_fields = 0;
    let mut count_records = 0;

    let mut count_empty = 0;

    loop {
        // We skip handling the output since we don't need it for counting.
        let (result, nin, _) = rdr.read_field(bytes, &mut [0; 1024]);
        bytes = &bytes[nin..];
        match result {
            ReadFieldResult::InputEmpty => {
                count_empty += 1;
            }
            ReadFieldResult::OutputFull => panic!("field too large"),
            ReadFieldResult::Field { record_end } => {
                count_fields += 1;
                if record_end {
                    count_records += 1;
                }
            }
            ReadFieldResult::End => break,
        }
    }
    assert_eq!(3, count_records);
    assert_eq!(9, count_fields);
    assert_eq!(0, count_empty);
}
