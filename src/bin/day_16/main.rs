type BitSlice<'a> = &'a bitvec::slice::BitSlice<bitvec::order::Msb0, u8>;

fn main() {
    let input = Packet::new(include_str!("../../../input/day_16_input.txt"));

    assert_eq!(16, dbg!(part_1(&Packet::new("8A004A801A8002F478"))));
    assert_eq!(12, dbg!(part_1(&Packet::new("620080001611562C8802118E34"))));
    assert_eq!(
        23,
        dbg!(part_1(&Packet::new("C0015000016115A2E0802F182340")))
    );
    assert_eq!(
        31,
        dbg!(part_1(&Packet::new("A0016C880162017C3686B18A3D4780")))
    );
    dbg!(part_1(&input));

    assert_eq!(3, dbg!(part_2(&Packet::new("C200B40A82"))));
    assert_eq!(54, dbg!(part_2(&Packet::new("04005AC33890"))));
    assert_eq!(7, dbg!(part_2(&Packet::new("880086C3E88112"))));
    assert_eq!(9, dbg!(part_2(&Packet::new("CE00C43D881120"))));
    assert_eq!(1, dbg!(part_2(&Packet::new("D8005AC2A8F0"))));
    assert_eq!(0, dbg!(part_2(&Packet::new("F600BC2D8F"))));
    assert_eq!(0, dbg!(part_2(&Packet::new("9C005AC2F8F0"))));
    assert_eq!(1, dbg!(part_2(&Packet::new("9C0141080250320F1802104A08"))));
    dbg!(part_2(&input));
}

fn part_1(packet: &Packet) -> usize {
    fn sum_version(packet: &Packet) -> usize {
        packet.version as usize
            + match &packet.ty {
                PacketType::Operator { children, .. } => children.iter().map(sum_version).sum(),
                _ => 0,
            }
    }

    sum_version(packet)
}

fn part_2(packet: &Packet) -> usize {
    fn inner(packet: &Packet) -> usize {
        match &packet.ty {
            PacketType::Literal(lit) => *lit,
            PacketType::Operator { ty, children } => {
                let mut children = children.iter().map(inner);
                match ty {
                    OperatorType::Sum => children.sum(),
                    OperatorType::Maximum => children.max().unwrap(),
                    OperatorType::Minimum => children.min().unwrap(),
                    OperatorType::Product => children.product(),
                    OperatorType::GreaterThan => {
                        let (first, second) = (children.next().unwrap(), children.next().unwrap());
                        if first > second {
                            1
                        } else {
                            0
                        }
                    }
                    OperatorType::LessThan => {
                        let (first, second) = (children.next().unwrap(), children.next().unwrap());
                        if first < second {
                            1
                        } else {
                            0
                        }
                    }
                    OperatorType::EqualTo => {
                        let (first, second) = (children.next().unwrap(), children.next().unwrap());
                        if first == second {
                            1
                        } else {
                            0
                        }
                    }
                    OperatorType::Unknown(op) => panic!("Unknown operator {}", op),
                }
            }
        }
    }

    inner(packet)
}

#[derive(Clone, Debug, PartialEq)]
struct Packet {
    pub version: u8,
    pub ty: PacketType,
}

impl Packet {
    fn new(input: &'static str) -> Self {
        let decoded = hex::decode(input.trim()).unwrap();
        let mut input = bitvec::view::BitView::view_bits(decoded.as_slice());
        Self::decode(&mut input)
    }

    fn decode(input: &mut BitSlice) -> Self {
        let version = take_u8(input, 3);
        let ty = match take_u8(input, 3) {
            4 => PacketType::literal(input),
            x => {
                let ty = OperatorType::new(x);
                let children = Self::parse_children(input);
                PacketType::Operator { ty, children }
            }
        };

        Self { version, ty }
    }

    fn parse_children(input: &mut BitSlice) -> Vec<Packet> {
        let mut children = Vec::new();
        let (bit, remaining) = input.split_first().unwrap();
        *input = remaining;
        if *bit {
            // next 11 bits are # of children
            let num = take_usize(input, 11);
            children.reserve_exact(num);
            for _ in 0..num {
                children.push(Packet::decode(input));
            }
        } else {
            // next 15 bits are subslice len
            let len = take_usize(input, 15);
            let (mut slice, remaining) = input.split_at(len);
            *input = remaining;
            while !slice.is_empty() {
                children.push(Packet::decode(&mut slice));
            }
        }
        children
    }
}

#[derive(Clone, Debug, PartialEq)]
enum PacketType {
    Literal(usize),
    Operator {
        ty: OperatorType,
        children: Vec<Packet>,
    },
}

#[derive(Clone, Debug, PartialEq)]
enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
    Unknown(u8),
}

impl OperatorType {
    fn new(val: u8) -> Self {
        match val {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Minimum,
            3 => Self::Maximum,
            5 => Self::GreaterThan,
            6 => Self::LessThan,
            7 => Self::EqualTo,
            val => Self::Unknown(val),
        }
    }
}

impl PacketType {
    fn literal(slice: &mut BitSlice) -> Self {
        let mut result = 0;
        loop {
            let (var, s) = slice.split_at(5);
            *slice = s;

            result = result << 4 | to_u8(&var[1..]) as usize;

            let first_bit_is_high = *var.first().unwrap();
            if !first_bit_is_high {
                // if the first bit is 0, this is the last value
                break;
            }
        }

        Self::Literal(result)
    }
}

fn take_u8(slice: &mut BitSlice, idx: usize) -> u8 {
    let (var, remaining) = slice.split_at(idx);
    *slice = remaining;
    to_u8(var)
}

fn to_u8(slice: BitSlice) -> u8 {
    let mut result = 0;
    for bit in slice {
        result = result << 1 | if *bit { 1 } else { 0 };
    }
    result
}

fn take_usize(slice: &mut BitSlice, idx: usize) -> usize {
    let (var, remaining) = slice.split_at(idx);
    *slice = remaining;
    to_usize(var)
}

fn to_usize(slice: BitSlice) -> usize {
    let mut result = 0;
    for bit in slice {
        result = result << 1 | if *bit { 1 } else { 0 };
    }
    result
}

#[test]
fn test_parse() {
    fn lit(version: u8, val: usize) -> Packet {
        Packet {
            version,
            ty: PacketType::Literal(val),
        }
    }
    fn op(version: u8, ty: OperatorType, children: Vec<Packet>) -> Packet {
        Packet {
            version,
            ty: PacketType::Operator { ty, children },
        }
    }

    assert_eq!(Packet::new("D2FE28"), lit(6, 2021));
    assert_eq!(
        Packet::new("38006F45291200"),
        op(1, OperatorType::LessThan, vec![lit(6, 10), lit(2, 20),])
    );
    assert_eq!(
        Packet::new("EE00D40C823060"),
        op(
            7,
            OperatorType::Maximum,
            vec![lit(2, 1), lit(4, 2), lit(1, 3),]
        )
    );

    assert_eq!(
        Packet::new("8A004A801A8002F478"),
        op(
            4,
            OperatorType::Minimum,
            vec![op(
                1,
                OperatorType::Minimum,
                vec![op(5, OperatorType::Minimum, vec![lit(6, 15)])]
            )]
        )
    );

    assert_eq!(
        Packet::new("620080001611562C8802118E34"),
        op(
            3,
            OperatorType::Sum,
            vec![
                op(0, OperatorType::Sum, vec![lit(0, 10), lit(5, 11)]),
                op(1, OperatorType::Sum, vec![lit(0, 12), lit(3, 13)]),
            ]
        )
    );

    assert_eq!(
        Packet::new("C0015000016115A2E0802F182340"),
        op(
            6,
            OperatorType::Sum,
            vec![
                op(0, OperatorType::Sum, vec![lit(0, 10), lit(6, 11)]),
                op(4, OperatorType::Sum, vec![lit(7, 12), lit(0, 13)]),
            ]
        )
    );

    assert_eq!(
        Packet::new("A0016C880162017C3686B18A3D4780"),
        op(
            5,
            OperatorType::Sum,
            vec![op(
                1,
                OperatorType::Sum,
                vec![op(
                    3,
                    OperatorType::Sum,
                    vec![lit(7, 6), lit(6, 6), lit(5, 12), lit(2, 15), lit(2, 15),]
                )]
            )]
        )
    );
}
