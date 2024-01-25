use three_d::{vec3, vec2,Srgba,Positions,Indices,Vector3, Vector2,CpuMesh};
use std::f32::consts::PI;


fn fprime(theta:f32,cost:f32)->f32{
    let st = theta.sin();
    ((cost*cost - st*st).abs()).sqrt() / st
}
    
const D_THETA : f32 = 0.001;

pub fn f(theta:f32, t:f32) -> f32{
    let mut th_curr = PI*0.5f32 - t;
    let mut ftotal = 0f32;
    let cost = t.cos();
        
    while th_curr > theta {
        ftotal -= fprime(th_curr,cost) * D_THETA;

        th_curr -= D_THETA;
    }

    ftotal
}
    
    



pub fn linspace(x0: f32, xend: f32, n: usize) -> Vec<f32> {
    let to_float = |i: usize| (i as f32);
    let dx = (xend - x0) / to_float(n - 1);
    (0..n).map(|i| x0 + to_float(i) * dx).collect()
}

const N_U : usize = 256;
const N_V : usize = 32;
const N_VERTS : usize = N_U * N_V;

const HEIGHT : f32 = 2.5;

pub fn make_dini(t : f32)->CpuMesh{
    //let a = t.cos();
    let b = t.sin();

    let b_over_a = t.tan();

    let maxu = HEIGHT/b;
    let us : Vec<f32> = linspace(-maxu,maxu,N_U);
    let vs : Vec<f32> = linspace(0.01f32,PI/2f32,N_V);



    let mut xyz : Vec<Vector3<f32>> = Vec::with_capacity(N_VERTS);

    

    let mut uv_coords : Vec<Vector2<f32>> = Vec::with_capacity(N_VERTS);
    let mut base_colors : Vec<Srgba> = Vec::with_capacity(N_VERTS);

    // println!("yo look {}",((PI*0.5).sin()/t.tan()).atan() /(PI*0.5-t));

    for vi in 0..N_V{
        let v = vs.get(vi).unwrap();
        let theta = (v.sin()/t.tan()).atan();
        let ftheta = f(theta,t);
        for ui in 0..N_U{
        
            let u = us.get(ui).unwrap();
            let logrho = u*(t.sin()) + ftheta;
            let rho = (logrho).exp();
            

            xyz.push(vec3(
                u.cos() * v.sin(),
                (v.cos() + (v*0.5f32).tan().ln()) + b_over_a*u,
                u.sin() * v.sin()
                
            ));

            uv_coords.push(vec2(
                rho,
                 (theta+ 4f32*PI) % (2f32*PI)
            ));

            base_colors.push(Srgba::new(
                (255f32*(ui as f32)/(N_U as f32)) as u8,
                (255f32*(vi as f32)/(N_V as f32)) as u8,
                0,
                255
            ));
        }
    }

    let pos : Positions = Positions::F32(xyz);
    let mut tris : Vec<usize> = Vec::with_capacity((N_U-1)*(N_V-1)*2*3);
    

    for vi in 0..N_V-1{
        for ui in 0..N_U-1{
        
            let tl = vi*N_U + ui;
            tris.push(tl);
            tris.push(tl+1);
            tris.push(tl+N_U);

            tris.push(tl+1);
            tris.push(tl+N_U+1);
            tris.push(tl+N_U);
        }
    }

    let tris32 = tris.iter().map(|&i| i as u32).collect();
    let indices = Indices::U32(tris32);

    CpuMesh {
        positions : pos,
        indices : indices,
        uvs : Some(uv_coords),
        colors : Some(base_colors),
        ..Default::default()
    }
}