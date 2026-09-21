use figures::{ink, save, Board};
use mrlyrs::core::error::Result;
use mrlyrs::math::graph::{census, roles, Role};
use mrlyrs::math::two::core_graph;
use mrlyrs::math::two::designs;

const LEVEL: usize = 3;
const SIDE: f64 = 8.0;

fn main() -> Result<()> {
    let cell = designs::create(7, 2, LEVEL, 0, 2)?;
    let network = core_graph(&cell)?;
    let tally = census(&network);
    let tags = roles(&network);
    let tips = tags.iter().filter(|r| **r == Role::Tip).count();
    let through = tags.iter().filter(|r| **r == Role::Through).count();
    let junctions = tags.iter().filter(|r| **r == Role::Junction).count();
    assert_eq!(tally.nodes, 27);
    assert_eq!(tally.branches, 26);
    assert_eq!(tally.components, 1);
    assert_eq!((tips, through, junctions), (10, 9, 8));

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
            unit * 0.16,
            ink::fade(ink::blue(), 0.6),
        );
    }
    for (index, role) in tags.iter().enumerate() {
        let (x, y) = at(index);
        let (radius, color) = match role {
            Role::Junction => (unit * 0.30, ink::pink()),
            Role::Tip => (unit * 0.26, ink::yellow()),
            _ => (unit * 0.21, ink::blue()),
        };
        board.disc(x, y, radius, color);
    }
    save("wiki-graphs", &board)?;
    Ok(())
}
