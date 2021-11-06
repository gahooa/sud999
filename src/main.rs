
//mod time;
//mod scratch;
//mod suduko;
mod lua;

fn main() {

    //scratch::run();
    //suduko::run();
    lua::run().expect("fatal");

}
