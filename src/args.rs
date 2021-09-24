use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "kipp")]
pub struct Math {
    #[structopt(short, long)]
    pub one: usize,

    #[structopt(short, long)]
    pub two: usize,

    #[structopt(short, long)]
    pub sus: String

}