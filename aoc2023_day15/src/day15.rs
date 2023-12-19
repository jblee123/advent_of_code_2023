fn hash(s: &str) -> u8 {
    let mut hash = 0 as u32;
    s.as_bytes().iter().for_each(|c| {
        hash += *c as u32;
        hash *= 17;
        hash %= 256
    });

    hash as u8
}

pub fn parse_and_sum_step_hashes(s: &str) -> u32 {
    s.trim().split(',').map(|part| hash(part) as u32).sum()
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
struct Slot {
    label: String,
    lens: u32,
}

enum Instr {
    Insert(String, u32),
    Remove(String),
}

fn parse_instr(s: &str) -> Instr {
    if let Some(idx) = s.find('=') {
        let label = s[0..idx].to_string();
        let lens = s[idx + 1..].parse().unwrap();
        Instr::Insert(label, lens)
    } else if let Some(idx) = s.find('-') {
        let label = s[0..idx].to_string();
        Instr::Remove(label)
    } else {
        panic!("bad instruction string");
    }
}

fn do_insert(label: &str, lens: u32, slots: &mut Vec<Slot>) {
    if let Some(idx) = slots.iter().position(|slot| slot.label == label) {
        slots[idx].lens = lens;
    } else {
        slots.push(Slot {
            label: label.to_string(),
            lens,
        });
    }
}

fn do_delete(label: &str, slots: &mut Vec<Slot>) {
    if let Some(idx) = slots.iter().position(|slot| slot.label == label) {
        slots.remove(idx);
    }
}

fn do_instr(instr: &Instr, boxes: &mut Vec<Vec<Slot>>) {
    match instr {
        Instr::Insert(label, lens) => {
            let hash = hash(label);
            do_insert(label, *lens, &mut boxes[hash as usize]);
        }
        Instr::Remove(label) => {
            let hash = hash(label);
            do_delete(label, &mut boxes[hash as usize]);
        }
    }
}

pub fn process_input(s: &str) -> u64 {
    let instructions = s.trim().split(',');

    let mut boxes = vec![Vec::<Slot>::new(); 256];

    instructions.for_each(|instr| {
        do_instr(&parse_instr(instr), &mut boxes);
    });

    let mut focus_power = 0 as u64;
    boxes.iter().enumerate().for_each(|(box_idx, the_box)| {
        the_box.iter().enumerate().for_each(|(slot_idx, slot)| {
            let box_num = (box_idx + 1) as u64;
            let slot_num = (slot_idx + 1) as u64;
            let lens_power = slot.lens as u64;
            focus_power += box_num * slot_num * lens_power;
        });
    });

    focus_power
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);

        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
        assert_eq!(hash("qp=3"), 97);
        assert_eq!(hash("cm=2"), 47);
        assert_eq!(hash("qp-"), 14);
        assert_eq!(hash("pc=4"), 180);
        assert_eq!(hash("ot=9"), 9);
        assert_eq!(hash("ab=5"), 197);
        assert_eq!(hash("pc-"), 48);
        assert_eq!(hash("pc=6"), 214);
        assert_eq!(hash("ot=7"), 231);
    }

    #[test]
    fn test_parse_and_sum_step_hashes() {
        assert_eq!(parse_and_sum_step_hashes(SAMPLE_INPUT_1), 1320);
    }

    #[test]
    fn test_process_input() {
        assert_eq!(process_input(SAMPLE_INPUT_1), 145);
    }
}
