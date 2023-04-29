let data= [#{
	type:"water",
	reflectivity: 0.5,
	fresnel_strength: 1.5,
	wave_speed: 0.06,
	use_reflection: true,
	use_refraction: true,
}
];

fn make_white(mesh, pos){
    return #{
		type : "mesh",
		position : pos,
		scale: [20,20,20],
                texture: "assets/textures/Wood_Plywood_Front_001_basecolor.jpg",
                normal: "assets/textures/Wood_Plywood_Front_001_normal.jpg",
		mesh : mesh, 
    };
}
fn make_black(mesh, pos){
    return #{
		type : "mesh",
		position : pos,
		scale: [20,20,20],
                texture: "assets/textures/Wood_023_basecolor.jpg",
                normal: "assets/textures/Wood_023_normal.jpg",
		mesh : mesh, 
    };
}

fn make_side(offset, dir, maker){
    let data= [];

    for i in 0..8{
        let p = maker.call("assets/models/pawn.glb", [i,0,offset+dir]);

	data.push(p);
    }

    data.push(maker.call("assets/models/castle.glb", [0,0,offset]));
    data.push(maker.call("assets/models/castle.glb", [7,0,offset]));

    data.push(maker.call("assets/models/bishop.glb", [2,0,offset]));
    data.push(maker.call("assets/models/bishop.glb", [5,0,offset]));

    data.push(maker.call("assets/models/knight.glb", [1,0,offset]));
    data.push(maker.call("assets/models/knight.glb", [6,0,offset]));

    data.push(maker.call("assets/models/king.glb", [3,0,offset]));
    data.push(maker.call("assets/models/queen.glb", [4,0,offset]));

    return data;
}

data.append(make_side(0,1, make_white));
data.append(make_side(8,-1, make_black));

return data;