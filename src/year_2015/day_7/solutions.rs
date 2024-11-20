#![allow(unused_mut, unused_variables, dead_code)]
use std::cell::RefCell;
use std::collections::HashMap;

pub trait Signal {
    fn get_signal(&self, wire_value: &mut HashMap::<String, u16>) -> u16;
}

#[derive(Debug)]
pub enum Connection<'a> {
    And(&'a RefCell<Wire<'a>>, &'a RefCell<Wire<'a>>),
    AndValue(&'a RefCell<Wire<'a>>, u16),
    Or(&'a RefCell<Wire<'a>>, &'a RefCell<Wire<'a>>),
    LShift(&'a RefCell<Wire<'a>>, u8),
    RShift(&'a RefCell<Wire<'a>>, u8),
    Not(&'a RefCell<Wire<'a>>),
    Wire(&'a RefCell<Wire<'a>>),
    Value(u16),
    Nothing,
}

impl<'a> Signal for Connection<'a> {
    fn get_signal(&self, wire_value: &mut HashMap::<String, u16>) -> u16 {
        match self {
            Connection::And(wire_1, wire_2) => wire_1.borrow().get_signal(wire_value) & wire_2.borrow().get_signal(wire_value),
            Connection::AndValue(wire, val) => wire.borrow().get_signal(wire_value) & val,
            Connection::Or(wire_1, wire_2) => wire_1.borrow().get_signal(wire_value) | wire_2.borrow().get_signal(wire_value),
            Connection::LShift(wire, shift) => wire.borrow().get_signal(wire_value) << *shift,
            Connection::RShift(wire, shift) => wire.borrow().get_signal(wire_value) >> *shift,
            Connection::Not(wire) => !wire.borrow().get_signal(wire_value),
            Connection::Wire(wire) => wire.borrow().get_signal(wire_value),
            Connection::Value(val) => *val,
            Connection::Nothing => 0,
        }
    }
}

#[derive(Debug)]
pub struct Wire<'a> {
    pub name: String,
    pub signal: Option<u16>,
    pub connection: Box<Connection<'a>>
}

impl<'a> Wire<'a> {
    fn display_connection(&self) -> String {
        let connection: &Box<Connection<'a>> = &self.connection;
        match **connection {
            Connection::And(wire_1, wire_2) => format!("{} AND {}", wire_1.borrow().name, wire_2.borrow().name),
            Connection::AndValue(wire, val) => format!("{} AND {}", val, wire.borrow().name),
            Connection::Or(wire_1, wire_2) => format!("{} OR {}", wire_1.borrow().name, wire_2.borrow().name),
            Connection::LShift(wire, shift) => format!("{} LSHIFT {}", wire.borrow().name, shift),
            Connection::RShift(wire, shift) => format!("{} RSHIFT {}", wire.borrow().name, shift),
            Connection::Not(wire) => format!("NOT {}", wire.borrow().name),
            Connection::Wire(wire) => format!("{}", wire.borrow().name),
            Connection::Value(val) => format!("{}", val),
            Connection::Nothing => format!("Nothing"),
        }
    }
}

impl<'a> Signal for Wire<'a> {
    fn get_signal(&self, wire_value: &mut HashMap::<String, u16>) -> u16 {
        if let Some(signal) = self.signal {
            return signal;
        }
		
		match wire_value.get(&self.name.clone()) {
			Some(signal) => return *signal,
			None => {},
		}
		
        let signal: u16 = self.connection.get_signal(wire_value);
		wire_value.insert(self.name.clone(), signal);
		signal
    }
}

pub fn get_wire_by_name<'a>
(wire_list: &'a Vec<RefCell<Wire<'a>>>, name: &str) -> Option<&'a RefCell<Wire<'a>>> {
    for wire in wire_list {
        if wire.borrow().name == name {
            return Some(wire)
        }
    }

    None
}

pub fn set_connection<'a>
(wire_list: &'a Vec<RefCell<Wire<'a>>>, instruction: &str) {
    let line_split: Vec<&str> =
        instruction
        .split_whitespace()
        .collect();
    
    //println!("{instruction}");
    
    if instruction.contains("AND") {
        let left_arg: &str = line_split[0];
        
        match left_arg.parse::<u16>() {
            Ok(val) => {
                let right_wire: &RefCell<Wire> = get_wire_by_name(wire_list, line_split[2]).unwrap();
                let dest_wire: &RefCell<Wire> = get_wire_by_name(wire_list, line_split[4]).unwrap();
                let connection = Connection::AndValue(right_wire, val);
                dest_wire.borrow_mut().connection = Box::new(connection);
            },
            Err(_) => {
                let left_wire: &RefCell<Wire> = get_wire_by_name(wire_list, left_arg).unwrap();
                let right_wire: &RefCell<Wire> = get_wire_by_name(wire_list, line_split[2]).unwrap();
                let dest_wire: &RefCell<Wire> = get_wire_by_name(wire_list, line_split[4]).unwrap();
                let connection = Connection::And(left_wire, right_wire);
                dest_wire.borrow_mut().connection = Box::new(connection);
            }
        }
    }
    else if instruction.contains("OR") {
        let left_wire: &RefCell<Wire> = get_wire_by_name(wire_list, line_split[0]).unwrap();
        let right_wire: &RefCell<Wire> = get_wire_by_name(wire_list, line_split[2]).unwrap();
        let dest_wire: &RefCell<Wire> = get_wire_by_name(wire_list, line_split[4]).unwrap();
        let connection = Connection::Or(left_wire, right_wire);
        dest_wire.borrow_mut().connection = Box::new(connection);
    }
    else if instruction.contains("LSHIFT") {
        let wire: &RefCell<Wire> = get_wire_by_name(wire_list, line_split[0]).unwrap();
        let shift: u8 = line_split[2].parse::<u8>().unwrap();
        let dest_wire: &RefCell<Wire> = get_wire_by_name(wire_list, line_split[4]).unwrap();
        let connection = Connection::LShift(wire, shift);
        dest_wire.borrow_mut().connection = Box::new(connection);
    }
    else if instruction.contains("RSHIFT") {
        let wire: &RefCell<Wire> = get_wire_by_name(wire_list, line_split[0]).unwrap();
        let shift: u8 = line_split[2].parse::<u8>().unwrap();
        let dest_wire: &RefCell<Wire> = get_wire_by_name(wire_list, line_split[4]).unwrap();
        let connection = Connection::RShift(wire, shift);
        dest_wire.borrow_mut().connection = Box::new(connection);
    }
    else if instruction.contains("NOT") {
        let wire: &RefCell<Wire> = get_wire_by_name(wire_list, line_split[1]).unwrap();
        let dest_wire: &RefCell<Wire> = get_wire_by_name(wire_list, line_split[3]).unwrap();
        let connection = Connection::Not(wire);
        dest_wire.borrow_mut().connection = Box::new(connection);
    }
    else {
        let left_arg: &str = line_split[0];
        
        match left_arg.parse::<u16>() {
            Ok(val) => {
                let dest_wire: &RefCell<Wire> = get_wire_by_name(wire_list, line_split[2]).unwrap();
                let connection = Connection::Value(val);
                dest_wire.borrow_mut().connection = Box::new(connection);
            },
            Err(_) => {
                let direct_wire: &RefCell<Wire> = get_wire_by_name(wire_list, line_split[0]).unwrap();
                let dest_wire: &RefCell<Wire> = get_wire_by_name(wire_list, line_split[2]).unwrap();
                let connection = Connection::Wire(direct_wire);
                dest_wire.borrow_mut().connection = Box::new(connection);
            }
        }
    }
}

fn incr_map(usage_map: &mut HashMap<String, u8>, name: &str) {
    let count: u8 = match usage_map.get(name) {
        Some(val) => *val,
        None => 0,
    };
    
    usage_map.insert(String::from(name), count+1);
}

fn count_wire_usage
(usage_map: &mut HashMap<String, u8>, instruction: &str) {
    let line_split: Vec<&str> =
        instruction
        .split_whitespace()
        .collect();
    
    if instruction.contains("AND") {
        let left_arg: &str = line_split[0];
        
        match left_arg.parse::<u16>() {
            Ok(val) => {
                incr_map(usage_map, line_split[2]);
            },
            Err(_) => {
                incr_map(usage_map, left_arg);
                incr_map(usage_map, line_split[2]);
            }
        }
    }
    else if instruction.contains("OR") {
        incr_map(usage_map, line_split[0]);
        incr_map(usage_map, line_split[2]);
    }
    else if instruction.contains("LSHIFT") {
        incr_map(usage_map, line_split[0]);
    }
    else if instruction.contains("RSHIFT") {
        incr_map(usage_map, line_split[0]);
    }
    else if instruction.contains("NOT") {
        incr_map(usage_map, line_split[1]);
    }
    else {
        let left_arg: &str = line_split[0];
        
        match left_arg.parse::<u16>() {
            Ok(val) => {
                incr_map(usage_map, line_split[2]);
            },
            Err(_) => {
               incr_map(usage_map, line_split[0]);
            }
        }
    }
}

fn get_wire_chain<'a>(wire_list: &'a Vec<RefCell<Wire<'a>>>, name: &str) -> Vec<String> {
    let mut wire: &RefCell<Wire> = get_wire_by_name(wire_list, name).unwrap();
    
    let mut wire_chain: Vec<String> = Vec::new();
    
    let mut b: bool = true;
    
    while b {
        wire_chain.push(wire.borrow().name.clone());
        
        let connection: &Box<Connection> = &wire.borrow().connection;
        match **connection {
            Connection::And(wire_1, wire_2) => wire = &wire_1,
            Connection::AndValue(w, val) => wire = &w,
            Connection::Or(wire_1, wire_2) => wire = &wire_1,
            Connection::LShift(w, shift) => wire = &w,
            Connection::RShift(w, shift) => wire = &w,
            Connection::Not(w) => wire = &w,
            Connection::Wire(w) => wire = &w,
            Connection::Value(val) => {
                wire_chain.push(format!("{}", val));
                b = false;
            },
            Connection::Nothing => {
                wire_chain.push(String::from("Nothing"));
                b = false;
            },
        }
        
    }
    
    wire_chain
}

pub const EXAMPLE: &str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

fn main() {
    println!("Hello, world!");

    let mut wire_list: Vec<RefCell<Wire>> = Vec::new();

    println!("Création des fils...");
    
    // Crée des fils sans connexion.
    for line in INPUT.lines() {
        let splits: Vec<&str> = line.split(" -> ").collect::<Vec<&str>>();

        let wire = Wire {
            name: splits[1].to_owned(),
            signal: None,
            connection: Box::new(Connection::Nothing),
        };

        wire_list.push(RefCell::new(wire));
    }
    
    println!("Connexions des fils...");
    
    INPUT.lines().for_each(|line| set_connection(&wire_list, line));
    
    //wire_list.iter().for_each(|wire| println!("{} -> {}", wire.borrow().display_connection(), wire.borrow().name));
    
    println!("Recherche la valeur d'un fil...");
    
	let mut wire_value: HashMap<String, u16> = HashMap::new();
	
	get_wire_by_name(&wire_list, "b").unwrap().borrow_mut().signal = Some(16076);
	println!("{}", get_wire_by_name(&wire_list, "a").unwrap().borrow().get_signal(&mut wire_value));
}


pub const INPUT: &str = "lf AND lq -> ls
iu RSHIFT 1 -> jn
bo OR bu -> bv
gj RSHIFT 1 -> hc
et RSHIFT 2 -> eu
bv AND bx -> by
is OR it -> iu
b OR n -> o
gf OR ge -> gg
NOT kt -> ku
ea AND eb -> ed
kl OR kr -> ks
hi AND hk -> hl
au AND av -> ax
lf RSHIFT 2 -> lg
dd RSHIFT 3 -> df
eu AND fa -> fc
df AND dg -> di
ip LSHIFT 15 -> it
NOT el -> em
et OR fe -> ff
fj LSHIFT 15 -> fn
t OR s -> u
ly OR lz -> ma
ko AND kq -> kr
NOT fx -> fy
et RSHIFT 1 -> fm
eu OR fa -> fb
dd RSHIFT 2 -> de
NOT go -> gp
kb AND kd -> ke
hg OR hh -> hi
jm LSHIFT 1 -> kg
NOT cn -> co
jp RSHIFT 2 -> jq
jp RSHIFT 5 -> js
1 AND io -> ip
eo LSHIFT 15 -> es
1 AND jj -> jk
g AND i -> j
ci RSHIFT 3 -> ck
gn AND gp -> gq
fs AND fu -> fv
lj AND ll -> lm
jk LSHIFT 15 -> jo
iu RSHIFT 3 -> iw
NOT ii -> ij
1 AND cc -> cd
bn RSHIFT 3 -> bp
NOT gw -> gx
NOT ft -> fu
jn OR jo -> jp
iv OR jb -> jc
hv OR hu -> hw
19138 -> b
gj RSHIFT 5 -> gm
hq AND hs -> ht
dy RSHIFT 1 -> er
ao OR an -> ap
ld OR le -> lf
bk LSHIFT 1 -> ce
bz AND cb -> cc
bi LSHIFT 15 -> bm
il AND in -> io
af AND ah -> ai
as RSHIFT 1 -> bl
lf RSHIFT 3 -> lh
er OR es -> et
NOT ax -> ay
ci RSHIFT 1 -> db
et AND fe -> fg
lg OR lm -> ln
k AND m -> n
hz RSHIFT 2 -> ia
kh LSHIFT 1 -> lb
NOT ey -> ez
NOT di -> dj
dz OR ef -> eg
lx -> a
NOT iz -> ja
gz LSHIFT 15 -> hd
ce OR cd -> cf
fq AND fr -> ft
at AND az -> bb
ha OR gz -> hb
fp AND fv -> fx
NOT gb -> gc
ia AND ig -> ii
gl OR gm -> gn
0 -> c
NOT ca -> cb
bn RSHIFT 1 -> cg
c LSHIFT 1 -> t
iw OR ix -> iy
kg OR kf -> kh
dy OR ej -> ek
km AND kn -> kp
NOT fc -> fd
hz RSHIFT 3 -> ib
NOT dq -> dr
NOT fg -> fh
dy RSHIFT 2 -> dz
kk RSHIFT 2 -> kl
1 AND fi -> fj
NOT hr -> hs
jp RSHIFT 1 -> ki
bl OR bm -> bn
1 AND gy -> gz
gr AND gt -> gu
db OR dc -> dd
de OR dk -> dl
as RSHIFT 5 -> av
lf RSHIFT 5 -> li
hm AND ho -> hp
cg OR ch -> ci
gj AND gu -> gw
ge LSHIFT 15 -> gi
e OR f -> g
fp OR fv -> fw
fb AND fd -> fe
cd LSHIFT 15 -> ch
b RSHIFT 1 -> v
at OR az -> ba
bn RSHIFT 2 -> bo
lh AND li -> lk
dl AND dn -> do
eg AND ei -> ej
ex AND ez -> fa
NOT kp -> kq
NOT lk -> ll
x AND ai -> ak
jp OR ka -> kb
NOT jd -> je
iy AND ja -> jb
jp RSHIFT 3 -> jr
fo OR fz -> ga
df OR dg -> dh
gj RSHIFT 2 -> gk
gj OR gu -> gv
NOT jh -> ji
ap LSHIFT 1 -> bj
NOT ls -> lt
ir LSHIFT 1 -> jl
bn AND by -> ca
lv LSHIFT 15 -> lz
ba AND bc -> bd
cy LSHIFT 15 -> dc
ln AND lp -> lq
x RSHIFT 1 -> aq
gk OR gq -> gr
NOT kx -> ky
jg AND ji -> jj
bn OR by -> bz
fl LSHIFT 1 -> gf
bp OR bq -> br
he OR hp -> hq
et RSHIFT 5 -> ew
iu RSHIFT 2 -> iv
gl AND gm -> go
x OR ai -> aj
hc OR hd -> he
lg AND lm -> lo
lh OR li -> lj
da LSHIFT 1 -> du
fo RSHIFT 2 -> fp
gk AND gq -> gs
bj OR bi -> bk
lf OR lq -> lr
cj AND cp -> cr
hu LSHIFT 15 -> hy
1 AND bh -> bi
fo RSHIFT 3 -> fq
NOT lo -> lp
hw LSHIFT 1 -> iq
dd RSHIFT 1 -> dw
dt LSHIFT 15 -> dx
dy AND ej -> el
an LSHIFT 15 -> ar
aq OR ar -> as
1 AND r -> s
fw AND fy -> fz
NOT im -> in
et RSHIFT 3 -> ev
1 AND ds -> dt
ec AND ee -> ef
NOT ak -> al
jl OR jk -> jm
1 AND en -> eo
lb OR la -> lc
iu AND jf -> jh
iu RSHIFT 5 -> ix
bo AND bu -> bw
cz OR cy -> da
iv AND jb -> jd
iw AND ix -> iz
lf RSHIFT 1 -> ly
iu OR jf -> jg
NOT dm -> dn
lw OR lv -> lx
gg LSHIFT 1 -> ha
lr AND lt -> lu
fm OR fn -> fo
he RSHIFT 3 -> hg
aj AND al -> am
1 AND kz -> la
dy RSHIFT 5 -> eb
jc AND je -> jf
cm AND co -> cp
gv AND gx -> gy
ev OR ew -> ex
jp AND ka -> kc
fk OR fj -> fl
dy RSHIFT 3 -> ea
NOT bs -> bt
NOT ag -> ah
dz AND ef -> eh
cf LSHIFT 1 -> cz
NOT cv -> cw
1 AND cx -> cy
de AND dk -> dm
ck AND cl -> cn
x RSHIFT 5 -> aa
dv LSHIFT 1 -> ep
he RSHIFT 2 -> hf
NOT bw -> bx
ck OR cl -> cm
bp AND bq -> bs
as OR bd -> be
he AND hp -> hr
ev AND ew -> ey
1 AND lu -> lv
kk RSHIFT 3 -> km
b AND n -> p
NOT kc -> kd
lc LSHIFT 1 -> lw
km OR kn -> ko
id AND if -> ig
ih AND ij -> ik
jr AND js -> ju
ci RSHIFT 5 -> cl
hz RSHIFT 1 -> is
1 AND ke -> kf
NOT gs -> gt
aw AND ay -> az
x RSHIFT 2 -> y
ab AND ad -> ae
ff AND fh -> fi
ci AND ct -> cv
eq LSHIFT 1 -> fk
gj RSHIFT 3 -> gl
u LSHIFT 1 -> ao
NOT bb -> bc
NOT hj -> hk
kw AND ky -> kz
as AND bd -> bf
dw OR dx -> dy
br AND bt -> bu
kk AND kv -> kx
ep OR eo -> eq
he RSHIFT 1 -> hx
ki OR kj -> kk
NOT ju -> jv
ek AND em -> en
kk RSHIFT 5 -> kn
NOT eh -> ei
hx OR hy -> hz
ea OR eb -> ec
s LSHIFT 15 -> w
fo RSHIFT 1 -> gh
kk OR kv -> kw
bn RSHIFT 5 -> bq
NOT ed -> ee
1 AND ht -> hu
cu AND cw -> cx
b RSHIFT 5 -> f
kl AND kr -> kt
iq OR ip -> ir
ci RSHIFT 2 -> cj
cj OR cp -> cq
o AND q -> r
dd RSHIFT 5 -> dg
b RSHIFT 2 -> d
ks AND ku -> kv
b RSHIFT 3 -> e
d OR j -> k
NOT p -> q
NOT cr -> cs
du OR dt -> dv
kf LSHIFT 15 -> kj
NOT ac -> ad
fo RSHIFT 5 -> fr
hz OR ik -> il
jx AND jz -> ka
gh OR gi -> gj
kk RSHIFT 1 -> ld
hz RSHIFT 5 -> ic
as RSHIFT 2 -> at
NOT jy -> jz
1 AND am -> an
ci OR ct -> cu
hg AND hh -> hj
jq OR jw -> jx
v OR w -> x
la LSHIFT 15 -> le
dh AND dj -> dk
dp AND dr -> ds
jq AND jw -> jy
au OR av -> aw
NOT bf -> bg
z OR aa -> ab
ga AND gc -> gd
hz AND ik -> im
jt AND jv -> jw
z AND aa -> ac
jr OR js -> jt
hb LSHIFT 1 -> hv
hf OR hl -> hm
ib OR ic -> id
fq OR fr -> fs
cq AND cs -> ct
ia OR ig -> ih
dd OR do -> dp
d AND j -> l
ib AND ic -> ie
as RSHIFT 3 -> au
be AND bg -> bh
dd AND do -> dq
NOT l -> m
1 AND gd -> ge
y AND ae -> ag
fo AND fz -> gb
NOT ie -> if
e AND f -> h
x RSHIFT 3 -> z
y OR ae -> af
hf AND hl -> hn
NOT h -> i
NOT hn -> ho
he RSHIFT 5 -> hh";
