use pathfinding::prelude::astar;
use cpu_time::ProcessTime;

const DIM:usize=3;

#[repr(packed)]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct MyPos {
    mat: [[u8;DIM];DIM],
    i: u8,
    j: u8
}

impl MyPos {
    fn mydistance(&self) -> u32 {
	let mut sum:i32 = 0;
	for (i,v) in self.mat.iter().enumerate() {
	    for (j,x) in v.iter().enumerate() {
                if *x!=0_u8 {
		    let i1:i32=((*x as i32)-1) / (DIM as i32);
		    let j1:i32=((*x as i32)-1) % (DIM as i32);
		    sum+=(i1-(i as i32)).abs()+(j1-(j as i32)).abs();
                }
	    }
	}
	sum as u32
    }
    fn mysuccessors(&self) -> Vec<(Self, u32)> {
	let mut vec = Vec::new();
	for i in [-1,1] {
	    let ni = i+(self.i as i32);
	    if (ni>=0) && (ni<(DIM as i32)) {
		let ni = ni as usize;
                let nj = self.j as usize;
		let mut nv = self.clone();
		nv.mat[self.i as usize][self.j as usize]=nv.mat[ni][nj];
		nv.mat[ni][nj]=0;
		nv.i=ni as u8;
		vec.push((nv,1_u32));
            }
        }
	for j in [-1,1] {
	    let nj = j + (self.j as i32);
	    if (nj>=0) && (nj<(DIM as i32)) {
                let ni = self.i as usize;
		let nj = nj as usize;
		let mut nv = self.clone();
		nv.mat[self.i as usize][self.j as usize]=nv.mat[ni][nj];
		nv.mat[ni][nj]=0;
		nv.j=nj as u8;
		vec.push((nv,1_u32));
	    }
	}
	vec
    }
    fn normalize(&mut self) {
        for (i,v) in self.mat.iter().enumerate() {
	    for (j,x) in v.iter().enumerate() {
                if *x==0 {
                    self.i=i as u8;
                    self.j=j as u8;
                }
            }
        }
    }
}

fn build(v:Vec<usize>,n:usize,m:&mut MyPos,t:&mut Vec<u32>) {
    for (k,e) in v.iter().enumerate() {
        let mut v2 = v.clone();
        let _r = v2.remove(k);
        let i=n/DIM;
        let j=n%DIM;
        m.mat[i][j]= *e as u8;
        if n==(DIM*DIM-1) {
            m.normalize();
            let spt= ProcessTime::now();
            if let Some((_path,len)) = astar(
                m,
                |p| p.mysuccessors(),
                |p| p.mydistance(),
	        |p| p.mydistance()==0) {
                println!("{:?} {:?} {}",m.mat,spt.elapsed(),len);
                t[len as usize]+=1;
            }
            else {
                println!("{:?} {:?}",m.mat,spt.elapsed());
            }
        }
        else {
            build(v2,n+1,m,t);
        }
    }
}

fn main() {
    let mut tlen = vec![0_u32;40];
    let mut zero: MyPos = Default::default();
    let v: Vec<usize> = Vec::from_iter(0..DIM*DIM);
    build(v,0,&mut zero,&mut tlen);
    println!("{:?}",tlen);
}
