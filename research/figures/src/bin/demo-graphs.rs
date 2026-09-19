use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board};
use mrlymath::two::designs;
use mrlymath::two::graph::core_graph;
use mrlynum::graph::{census, roles, Role};

const LEVEL: usize = 5;
const SIDE: f64 = 32.0;

fn main() -> Result<()> {
    let cell = designs::create(7, 2, LEVEL, 0, 2)?;
    let network = core_graph(&cell)?;
    let tally = census(&network);
    let tags = roles(&network);
    let tips = tags.iter().filter(|r| **r == Role::Tip).count();
    let paths = tags.iter().filter(|r| **r == Role::Through).count();
    let junctions = tags.iter().filter(|r| **r == Role::Junction).count();
    assert_eq!(tally.nodes, 243);
    assert_eq!(tally.branches, 242);
    assert_eq!(tally.components, 1);
    assert_eq!((tips, paths, junctions), (82, 81, 80));

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let unit = frame.w / SIDE;
    let at = |index: usize| {
        let node = &network.nodes[index];
        (
            frame.x + node.position[0] * unit,
            frame.y + node.position[1] * unit,
        )
    };
    for branch in &network.branches {
        board.segment(
            at(branch.parent),
            at(branch.child),
            unit * 0.20,
            ink::fade(ink::blue(), 0.55),
        );
    }
    for (index, role) in tags.iter().enumerate() {
        let (x, y) = at(index);
        let (radius, color) = match role {
            Role::Junction => (unit * 0.34, ink::pink()),
            Role::Tip => (unit * 0.30, ink::yellow()),
            _ => (unit * 0.20, ink::blue()),
        };
        board.disc(x, y, radius, color);
    }
    save("demo-graphs", &board)?;
    Ok(())
}
