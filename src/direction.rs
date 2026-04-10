//! [`Direction`], [`Axis`], and [`AxisDirection`] — cardinal directions in 3D space.
//!
//! These mirror the vanilla `Direction`, `Direction.Axis`, and
//! `Direction.AxisDirection` enums used throughout the Minecraft protocol
//! for block faces, entity facings, and spatial queries.

use std::fmt;

use bytes::{Bytes, BytesMut};

use oxidized_codec::types::TypeError;
use oxidized_codec::varint;

// ── Direction ───────────────────────────────────────────────────────────

/// A cardinal direction in 3D space.
///
/// Values 0–5 map to Down, Up, North, South, West, East and are used
/// directly as wire IDs in many packets.
///
/// # Examples
///
/// ```
/// use oxidized_mc_types::Direction;
///
/// let dir = Direction::North;
/// assert_eq!(dir.opposite(), Direction::South);
/// assert_eq!(dir.name(), "north");
///
/// // Y-rotation mapping (vanilla compass)
/// assert_eq!(Direction::from_y_rot(0.0), Direction::South);
/// assert_eq!(Direction::from_y_rot(90.0), Direction::West);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Direction {
    /// Negative Y.
    Down = 0,
    /// Positive Y.
    Up = 1,
    /// Negative Z.
    North = 2,
    /// Positive Z.
    South = 3,
    /// Negative X.
    West = 4,
    /// Positive X.
    East = 5,
}

/// All six directions in order of their 3D data value.
pub const ALL: [Direction; 6] = [
    Direction::Down,
    Direction::Up,
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
];

/// The four horizontal directions: South, West, North, East (2D data-value order).
pub const HORIZONTALS: [Direction; 4] = [
    Direction::South,
    Direction::West,
    Direction::North,
    Direction::East,
];

impl Direction {
    /// Returns the opposite direction.
    pub fn opposite(self) -> Direction {
        match self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }

    /// Rotates this direction 90° clockwise around the Y axis.
    ///
    /// Only valid for horizontal directions.
    /// Returns `None` for [`Direction::Up`] and [`Direction::Down`].
    pub fn clockwise(self) -> Option<Direction> {
        match self {
            Direction::North => Some(Direction::East),
            Direction::East => Some(Direction::South),
            Direction::South => Some(Direction::West),
            Direction::West => Some(Direction::North),
            Direction::Up | Direction::Down => None,
        }
    }

    /// Rotates this direction 90° counter-clockwise around the Y axis.
    ///
    /// Only valid for horizontal directions.
    /// Returns `None` for [`Direction::Up`] and [`Direction::Down`].
    pub fn counter_clockwise(self) -> Option<Direction> {
        match self {
            Direction::North => Some(Direction::West),
            Direction::West => Some(Direction::South),
            Direction::South => Some(Direction::East),
            Direction::East => Some(Direction::North),
            Direction::Up | Direction::Down => None,
        }
    }

    /// X component of the direction's unit normal vector.
    pub fn step_x(self) -> i32 {
        match self {
            Direction::West => -1,
            Direction::East => 1,
            _ => 0,
        }
    }

    /// Y component of the direction's unit normal vector.
    pub fn step_y(self) -> i32 {
        match self {
            Direction::Down => -1,
            Direction::Up => 1,
            _ => 0,
        }
    }

    /// Z component of the direction's unit normal vector.
    pub fn step_z(self) -> i32 {
        match self {
            Direction::North => -1,
            Direction::South => 1,
            _ => 0,
        }
    }

    /// Returns the axis this direction lies on.
    pub fn axis(self) -> Axis {
        match self {
            Direction::Down | Direction::Up => Axis::Y,
            Direction::North | Direction::South => Axis::Z,
            Direction::West | Direction::East => Axis::X,
        }
    }

    /// Returns the axis direction (positive or negative) for this direction.
    pub fn axis_direction(self) -> AxisDirection {
        match self {
            Direction::Down | Direction::North | Direction::West => AxisDirection::Negative,
            Direction::Up | Direction::South | Direction::East => AxisDirection::Positive,
        }
    }

    /// Converts a 3D data value (0–5) to a [`Direction`].
    ///
    /// Returns `None` if `id` is out of range.
    pub fn from_3d_data_value(id: u8) -> Option<Direction> {
        match id {
            0 => Some(Direction::Down),
            1 => Some(Direction::Up),
            2 => Some(Direction::North),
            3 => Some(Direction::South),
            4 => Some(Direction::West),
            5 => Some(Direction::East),
            _ => None,
        }
    }

    /// Converts a 2D data value to a horizontal [`Direction`].
    ///
    /// Mapping: 0=South, 1=West, 2=North, 3=East.
    /// Returns `None` if `id` is out of range.
    pub fn from_2d_data_value(id: u8) -> Option<Direction> {
        match id {
            0 => Some(Direction::South),
            1 => Some(Direction::West),
            2 => Some(Direction::North),
            3 => Some(Direction::East),
            _ => None,
        }
    }

    /// Returns the 3D data value (0–5) for this direction.
    pub fn to_3d_data_value(self) -> u8 {
        self as u8
    }

    /// Returns the Y rotation in degrees for this horizontal direction.
    ///
    /// South=0, West=90, North=180, East=270.
    /// Returns 0.0 for vertical directions.
    pub fn to_y_rot(self) -> f32 {
        match self {
            Direction::South => 0.0,
            Direction::West => 90.0,
            Direction::North => 180.0,
            Direction::East => 270.0,
            Direction::Down | Direction::Up => 0.0,
        }
    }

    /// Converts a Y rotation (in degrees) to the nearest horizontal direction.
    ///
    /// South=0°, West=90°, North=180°, East=270°.
    pub fn from_y_rot(rot: f64) -> Direction {
        // Normalize to 0..360, then divide into 4 quadrants
        let normalized = ((rot % 360.0) + 360.0) % 360.0;
        let index = ((normalized + 45.0) / 90.0) as i32 & 3;
        match index {
            0 => Direction::South,
            1 => Direction::West,
            2 => Direction::North,
            3 => Direction::East,
            _ => Direction::South, // unreachable due to & 3
        }
    }

    /// Returns the lowercase name of this direction.
    pub fn name(self) -> &'static str {
        match self {
            Direction::Down => "down",
            Direction::Up => "up",
            Direction::North => "north",
            Direction::South => "south",
            Direction::West => "west",
            Direction::East => "east",
        }
    }

    /// Returns `true` if this is a horizontal direction (North/South/East/West).
    pub fn is_horizontal(self) -> bool {
        matches!(
            self,
            Direction::North | Direction::South | Direction::West | Direction::East
        )
    }

    /// Returns `true` if this is a vertical direction (Up/Down).
    pub fn is_vertical(self) -> bool {
        matches!(self, Direction::Up | Direction::Down)
    }

    /// Reads a [`Direction`] from a wire buffer as a VarInt (0–5).
    ///
    /// # Errors
    ///
    /// Returns [`TypeError`] if the buffer is truncated or the value is
    /// out of range.
    pub fn read(buf: &mut Bytes) -> Result<Self, TypeError> {
        let id = varint::read_varint_buf(buf)?;
        Direction::from_3d_data_value(id as u8).ok_or(TypeError::UnexpectedEof { need: 1, have: 0 })
    }

    /// Writes this [`Direction`] to a wire buffer as a VarInt.
    pub fn write(&self, buf: &mut BytesMut) {
        varint::write_varint_buf(i32::from(self.to_3d_data_value()), buf);
    }

    /// Returns the nearest direction to the given normal vector.
    ///
    /// Picks the direction whose unit normal has the largest dot product
    /// with the input. Ties are broken by the iteration order (Down first).
    pub fn get_nearest(x: f64, y: f64, z: f64) -> Direction {
        let mut best = Direction::Down;
        let mut best_dot = f64::MIN;
        for dir in ALL {
            let dot = x * f64::from(dir.step_x())
                + y * f64::from(dir.step_y())
                + z * f64::from(dir.step_z());
            if dot > best_dot {
                best_dot = dot;
                best = dir;
            }
        }
        best
    }
}

/// A logical grouping of directions (horizontal or vertical).
///
/// Matches vanilla's `Direction.Plane`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Plane {
    /// The four horizontal directions: North, South, East, West.
    Horizontal,
    /// The two vertical directions: Up, Down.
    Vertical,
}

impl Plane {
    /// Returns an iterator over the directions in this plane.
    pub fn directions(self) -> &'static [Direction] {
        match self {
            Plane::Horizontal => &HORIZONTALS,
            Plane::Vertical => &[Direction::Down, Direction::Up],
        }
    }

    /// Tests if the given direction belongs to this plane.
    pub fn test(self, direction: Direction) -> bool {
        match self {
            Plane::Horizontal => direction.is_horizontal(),
            Plane::Vertical => direction.is_vertical(),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

// ── Axis ────────────────────────────────────────────────────────────────

/// A coordinate axis in 3D space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
    /// The X axis (east/west).
    X,
    /// The Y axis (up/down).
    Y,
    /// The Z axis (north/south).
    Z,
}

impl Axis {
    /// Returns `true` if this is the Y axis.
    pub fn is_vertical(self) -> bool {
        self == Axis::Y
    }

    /// Returns `true` if this is the X or Z axis.
    pub fn is_horizontal(self) -> bool {
        matches!(self, Axis::X | Axis::Z)
    }

    /// Selects the component corresponding to this axis.
    pub fn choose<T>(self, x: T, y: T, z: T) -> T {
        match self {
            Axis::X => x,
            Axis::Y => y,
            Axis::Z => z,
        }
    }

    /// Returns the direction along the positive end of this axis.
    pub fn positive(self) -> Direction {
        match self {
            Axis::X => Direction::East,
            Axis::Y => Direction::Up,
            Axis::Z => Direction::South,
        }
    }

    /// Returns the direction along the negative end of this axis.
    pub fn negative(self) -> Direction {
        match self {
            Axis::X => Direction::West,
            Axis::Y => Direction::Down,
            Axis::Z => Direction::North,
        }
    }

    /// Returns the lowercase name of this axis.
    pub fn name(self) -> &'static str {
        match self {
            Axis::X => "x",
            Axis::Y => "y",
            Axis::Z => "z",
        }
    }
}

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

// ── AxisDirection ───────────────────────────────────────────────────────

/// Direction along an axis (positive or negative).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AxisDirection {
    /// The positive direction (+1).
    Positive,
    /// The negative direction (−1).
    Negative,
}

impl AxisDirection {
    /// Returns the integer step for this direction (+1 or −1).
    pub fn step(self) -> i32 {
        match self {
            AxisDirection::Positive => 1,
            AxisDirection::Negative => -1,
        }
    }

    /// Returns the opposite axis direction.
    pub fn opposite(self) -> AxisDirection {
        match self {
            AxisDirection::Positive => AxisDirection::Negative,
            AxisDirection::Negative => AxisDirection::Positive,
        }
    }
}

impl fmt::Display for AxisDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AxisDirection::Positive => f.write_str("positive"),
            AxisDirection::Negative => f.write_str("negative"),
        }
    }
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    // ── Construction ─────────────────────────────────────────────────

    #[test]
    fn test_direction_from_3d_data_value_valid() {
        assert_eq!(Direction::from_3d_data_value(0), Some(Direction::Down));
        assert_eq!(Direction::from_3d_data_value(1), Some(Direction::Up));
        assert_eq!(Direction::from_3d_data_value(2), Some(Direction::North));
        assert_eq!(Direction::from_3d_data_value(3), Some(Direction::South));
        assert_eq!(Direction::from_3d_data_value(4), Some(Direction::West));
        assert_eq!(Direction::from_3d_data_value(5), Some(Direction::East));
    }

    #[test]
    fn test_direction_from_3d_data_value_invalid() {
        assert_eq!(Direction::from_3d_data_value(6), None);
        assert_eq!(Direction::from_3d_data_value(255), None);
    }

    #[test]
    fn test_direction_from_2d_data_value() {
        assert_eq!(Direction::from_2d_data_value(0), Some(Direction::South));
        assert_eq!(Direction::from_2d_data_value(1), Some(Direction::West));
        assert_eq!(Direction::from_2d_data_value(2), Some(Direction::North));
        assert_eq!(Direction::from_2d_data_value(3), Some(Direction::East));
        assert_eq!(Direction::from_2d_data_value(4), None);
    }

    #[test]
    fn test_direction_to_3d_data_value() {
        for dir in ALL {
            assert_eq!(
                Direction::from_3d_data_value(dir.to_3d_data_value()),
                Some(dir)
            );
        }
    }

    // ── Opposite ─────────────────────────────────────────────────────

    #[test]
    fn test_direction_opposite_pairs() {
        assert_eq!(Direction::Down.opposite(), Direction::Up);
        assert_eq!(Direction::Up.opposite(), Direction::Down);
        assert_eq!(Direction::North.opposite(), Direction::South);
        assert_eq!(Direction::South.opposite(), Direction::North);
        assert_eq!(Direction::West.opposite(), Direction::East);
        assert_eq!(Direction::East.opposite(), Direction::West);
    }

    #[test]
    fn test_direction_double_opposite_is_identity() {
        for dir in ALL {
            assert_eq!(dir.opposite().opposite(), dir);
        }
    }

    // ── Clockwise rotation ──────────────────────────────────────────

    #[test]
    fn test_direction_clockwise_chain() {
        let mut dir = Direction::North;
        dir = dir.clockwise().unwrap();
        assert_eq!(dir, Direction::East);
        dir = dir.clockwise().unwrap();
        assert_eq!(dir, Direction::South);
        dir = dir.clockwise().unwrap();
        assert_eq!(dir, Direction::West);
        dir = dir.clockwise().unwrap();
        assert_eq!(dir, Direction::North);
    }

    #[test]
    fn test_direction_counter_clockwise_chain() {
        let mut dir = Direction::North;
        dir = dir.counter_clockwise().unwrap();
        assert_eq!(dir, Direction::West);
        dir = dir.counter_clockwise().unwrap();
        assert_eq!(dir, Direction::South);
        dir = dir.counter_clockwise().unwrap();
        assert_eq!(dir, Direction::East);
        dir = dir.counter_clockwise().unwrap();
        assert_eq!(dir, Direction::North);
    }

    #[test]
    fn test_direction_clockwise_vertical_returns_none() {
        assert_eq!(Direction::Up.clockwise(), None);
        assert_eq!(Direction::Down.clockwise(), None);
    }

    #[test]
    fn test_direction_counter_clockwise_vertical_returns_none() {
        assert_eq!(Direction::Up.counter_clockwise(), None);
        assert_eq!(Direction::Down.counter_clockwise(), None);
    }

    // ── Step vectors ────────────────────────────────────────────────

    #[test]
    fn test_direction_step_vectors() {
        assert_eq!(
            (
                Direction::Down.step_x(),
                Direction::Down.step_y(),
                Direction::Down.step_z()
            ),
            (0, -1, 0)
        );
        assert_eq!(
            (
                Direction::Up.step_x(),
                Direction::Up.step_y(),
                Direction::Up.step_z()
            ),
            (0, 1, 0)
        );
        assert_eq!(
            (
                Direction::North.step_x(),
                Direction::North.step_y(),
                Direction::North.step_z()
            ),
            (0, 0, -1)
        );
        assert_eq!(
            (
                Direction::South.step_x(),
                Direction::South.step_y(),
                Direction::South.step_z()
            ),
            (0, 0, 1)
        );
        assert_eq!(
            (
                Direction::West.step_x(),
                Direction::West.step_y(),
                Direction::West.step_z()
            ),
            (-1, 0, 0)
        );
        assert_eq!(
            (
                Direction::East.step_x(),
                Direction::East.step_y(),
                Direction::East.step_z()
            ),
            (1, 0, 0)
        );
    }

    // ── Y rotation ──────────────────────────────────────────────────

    #[test]
    fn test_direction_to_y_rot() {
        assert_eq!(Direction::South.to_y_rot(), 0.0);
        assert_eq!(Direction::West.to_y_rot(), 90.0);
        assert_eq!(Direction::North.to_y_rot(), 180.0);
        assert_eq!(Direction::East.to_y_rot(), 270.0);
    }

    #[test]
    fn test_direction_from_y_rot_exact() {
        assert_eq!(Direction::from_y_rot(0.0), Direction::South);
        assert_eq!(Direction::from_y_rot(90.0), Direction::West);
        assert_eq!(Direction::from_y_rot(180.0), Direction::North);
        assert_eq!(Direction::from_y_rot(270.0), Direction::East);
    }

    #[test]
    fn test_direction_from_y_rot_snapping() {
        assert_eq!(Direction::from_y_rot(44.0), Direction::South);
        assert_eq!(Direction::from_y_rot(46.0), Direction::West);
        assert_eq!(Direction::from_y_rot(-90.0), Direction::East);
        assert_eq!(Direction::from_y_rot(360.0), Direction::South);
        assert_eq!(Direction::from_y_rot(720.0), Direction::South);
    }

    // ── Axis ────────────────────────────────────────────────────────

    #[test]
    fn test_direction_axis() {
        assert_eq!(Direction::Down.axis(), Axis::Y);
        assert_eq!(Direction::Up.axis(), Axis::Y);
        assert_eq!(Direction::North.axis(), Axis::Z);
        assert_eq!(Direction::South.axis(), Axis::Z);
        assert_eq!(Direction::West.axis(), Axis::X);
        assert_eq!(Direction::East.axis(), Axis::X);
    }

    #[test]
    fn test_direction_axis_direction() {
        assert_eq!(Direction::Down.axis_direction(), AxisDirection::Negative);
        assert_eq!(Direction::Up.axis_direction(), AxisDirection::Positive);
        assert_eq!(Direction::North.axis_direction(), AxisDirection::Negative);
        assert_eq!(Direction::South.axis_direction(), AxisDirection::Positive);
        assert_eq!(Direction::West.axis_direction(), AxisDirection::Negative);
        assert_eq!(Direction::East.axis_direction(), AxisDirection::Positive);
    }

    // ── Horizontal / Vertical ───────────────────────────────────────

    #[test]
    fn test_direction_is_horizontal() {
        assert!(!Direction::Down.is_horizontal());
        assert!(!Direction::Up.is_horizontal());
        assert!(Direction::North.is_horizontal());
        assert!(Direction::South.is_horizontal());
        assert!(Direction::West.is_horizontal());
        assert!(Direction::East.is_horizontal());
    }

    #[test]
    fn test_direction_is_vertical() {
        assert!(Direction::Down.is_vertical());
        assert!(Direction::Up.is_vertical());
        assert!(!Direction::North.is_vertical());
    }

    // ── Name / Display ──────────────────────────────────────────────

    #[test]
    fn test_direction_name() {
        assert_eq!(Direction::Down.name(), "down");
        assert_eq!(Direction::Up.name(), "up");
        assert_eq!(Direction::North.name(), "north");
        assert_eq!(Direction::South.name(), "south");
        assert_eq!(Direction::West.name(), "west");
        assert_eq!(Direction::East.name(), "east");
    }

    #[test]
    fn test_direction_display() {
        assert_eq!(format!("{}", Direction::North), "north");
    }

    // ── Axis type ───────────────────────────────────────────────────

    #[test]
    fn test_axis_is_vertical() {
        assert!(!Axis::X.is_vertical());
        assert!(Axis::Y.is_vertical());
        assert!(!Axis::Z.is_vertical());
    }

    #[test]
    fn test_axis_is_horizontal() {
        assert!(Axis::X.is_horizontal());
        assert!(!Axis::Y.is_horizontal());
        assert!(Axis::Z.is_horizontal());
    }

    #[test]
    fn test_axis_choose() {
        assert_eq!(Axis::X.choose(10, 20, 30), 10);
        assert_eq!(Axis::Y.choose(10, 20, 30), 20);
        assert_eq!(Axis::Z.choose(10, 20, 30), 30);
    }

    #[test]
    fn test_axis_positive_negative() {
        assert_eq!(Axis::X.positive(), Direction::East);
        assert_eq!(Axis::X.negative(), Direction::West);
        assert_eq!(Axis::Y.positive(), Direction::Up);
        assert_eq!(Axis::Y.negative(), Direction::Down);
        assert_eq!(Axis::Z.positive(), Direction::South);
        assert_eq!(Axis::Z.negative(), Direction::North);
    }

    #[test]
    fn test_axis_name() {
        assert_eq!(Axis::X.name(), "x");
        assert_eq!(Axis::Y.name(), "y");
        assert_eq!(Axis::Z.name(), "z");
    }

    // ── AxisDirection type ──────────────────────────────────────────

    #[test]
    fn test_axis_direction_step() {
        assert_eq!(AxisDirection::Positive.step(), 1);
        assert_eq!(AxisDirection::Negative.step(), -1);
    }

    #[test]
    fn test_axis_direction_opposite() {
        assert_eq!(AxisDirection::Positive.opposite(), AxisDirection::Negative);
        assert_eq!(AxisDirection::Negative.opposite(), AxisDirection::Positive);
    }

    // ── Wire roundtrip ──────────────────────────────────────────────

    #[test]
    fn test_direction_wire_roundtrip() {
        for dir in ALL {
            let mut buf = BytesMut::new();
            dir.write(&mut buf);
            let mut data = buf.freeze();
            let decoded = Direction::read(&mut data).unwrap();
            assert_eq!(decoded, dir);
        }
    }

    // ── ALL / HORIZONTALS ───────────────────────────────────────────

    #[test]
    fn test_all_contains_six_directions() {
        assert_eq!(ALL.len(), 6);
    }

    #[test]
    fn test_horizontals_contains_four_directions() {
        assert_eq!(HORIZONTALS.len(), 4);
        for dir in HORIZONTALS {
            assert!(dir.is_horizontal());
        }
    }

    // ── get_nearest ─────────────────────────────────────────────────────

    #[test]
    fn test_direction_get_nearest_up() {
        assert_eq!(Direction::get_nearest(0.0, 1.0, 0.0), Direction::Up);
    }

    #[test]
    fn test_direction_get_nearest_down() {
        assert_eq!(Direction::get_nearest(0.0, -1.0, 0.0), Direction::Down);
    }

    #[test]
    fn test_direction_get_nearest_east() {
        assert_eq!(Direction::get_nearest(1.0, 0.0, 0.0), Direction::East);
    }

    #[test]
    fn test_direction_get_nearest_west() {
        assert_eq!(Direction::get_nearest(-1.0, 0.0, 0.0), Direction::West);
    }

    #[test]
    fn test_direction_get_nearest_diagonal() {
        // (1, 2, 0) → mostly up
        assert_eq!(Direction::get_nearest(1.0, 2.0, 0.0), Direction::Up);
    }

    // ── Plane ───────────────────────────────────────────────────────────

    #[test]
    fn test_plane_horizontal_directions() {
        let dirs = Plane::Horizontal.directions();
        assert_eq!(dirs.len(), 4);
        for d in dirs {
            assert!(d.is_horizontal());
        }
    }

    #[test]
    fn test_plane_vertical_directions() {
        let dirs = Plane::Vertical.directions();
        assert_eq!(dirs.len(), 2);
        for d in dirs {
            assert!(d.is_vertical());
        }
    }

    #[test]
    fn test_plane_test_membership() {
        assert!(Plane::Horizontal.test(Direction::North));
        assert!(!Plane::Horizontal.test(Direction::Up));
        assert!(Plane::Vertical.test(Direction::Up));
        assert!(!Plane::Vertical.test(Direction::East));
    }

    // ── Property-based tests ────────────────────────────────────────

    mod prop {
        use crate::direction::ALL;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn direction_opposite_involution(dir_idx in 0u8..6) {
                let dir = ALL[dir_idx as usize];
                prop_assert_eq!(dir.opposite().opposite(), dir);
            }

            #[test]
            fn direction_step_magnitude_one(dir_idx in 0u8..6) {
                let dir = ALL[dir_idx as usize];
                let sum = dir.step_x().abs() + dir.step_y().abs() + dir.step_z().abs();
                prop_assert_eq!(sum, 1);
            }
        }
    }

    // ── Compliance tests ────────────────────────────────────────────

    mod compliance {
        use super::*;

        #[test]
        fn compliance_direction_y_rotation_mapping() {
            assert_eq!(Direction::from_y_rot(0.0), Direction::South);
            assert_eq!(Direction::from_y_rot(90.0), Direction::West);
            assert_eq!(Direction::from_y_rot(180.0), Direction::North);
            assert_eq!(Direction::from_y_rot(270.0), Direction::East);
        }

        #[test]
        fn compliance_direction_y_rotation_wraps() {
            assert_eq!(Direction::from_y_rot(360.0), Direction::South);
            assert_eq!(Direction::from_y_rot(-90.0), Direction::East);
        }

        #[test]
        fn compliance_direction_names_lowercase() {
            for dir in &crate::direction::ALL {
                let name = dir.name();
                assert_eq!(name, name.to_lowercase());
            }
        }

        #[test]
        fn compliance_direction_display_matches_name() {
            for dir in &crate::direction::ALL {
                assert_eq!(dir.to_string(), dir.name());
            }
        }
    }

    // ── Snapshot tests ──────────────────────────────────────────────

    mod snapshots {
        use super::*;

        #[test]
        fn snapshot_direction_display() {
            insta::assert_snapshot!(Direction::North.to_string(), @"north");
            insta::assert_snapshot!(Direction::South.to_string(), @"south");
            insta::assert_snapshot!(Direction::East.to_string(), @"east");
            insta::assert_snapshot!(Direction::West.to_string(), @"west");
            insta::assert_snapshot!(Direction::Up.to_string(), @"up");
            insta::assert_snapshot!(Direction::Down.to_string(), @"down");
        }
    }
}
