use pollster;
use nocterminal::run;

fn main() {
    pollster::block_on(run());
}
