#![allow(dead_code)]
#![allow(unused_variables)]

mod collada;
mod file_ext;

use anyhow::*;
use collada::*;
use file_ext::*;
use nalgebra_glm::*;
use std::env::*;
use std::fs::*;
use std::io::*;
use std::path::*;

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = args().collect();
    let mut file = File::open(&args[1]).unwrap();

    let magic = file.read_u32()?;
    let num_mesh = file.read_u16()?;
    let num_material = file.read_u16()?;
    let num_light = file.read_u16()?;
    let num_camera = file.read_u16()?;
    let num_bones = file.read_u16()?;
    let num_unk = file.read_u16()?;

    let zero10 = file.read_u32()?;
    let center = file.read_f32vec3()?;
    let bound = file.read_f32vec3()?;
    let zero2c = file.read_u32()?;

    let unk30 = file.read_u32()?;
    let zero34 = file.read_u32()?;
    let unk38 = file.read_u32()?;
    let zero3c = file.read_u32()?;

    let material_id = file.read_u32()?;
    let unk44 = file.read_u32()?;
    let offset_mesh = file.read_u64()?; // 0x356284 ~ 0x358140
    let offset_material = file.read_u64()?;
    let offset_light = file.read_u64()?; // 0
    let offset_camera = file.read_u64()?; // 0
    let offset_inv_matrix = file.read_u64()?;
    let offset_unk70 = file.read_u64()?; // 0
    let offset_string_table = file.read_u64()?;
    let offset_unk80 = file.read_u64()?; // unexplored, 0x3588c0 ~ 0x358cdc
    let offset_unk88 = file.read_u64()?; // 0
    let offset_name = file.read_u64()?;
    let offset_tree = file.read_u64()?;
    let offset_unka0 = file.read_u64()?; // 0

    if num_light != 0 {
        bail!("Can't deal with light")
    }
    if num_camera != 0 {
        bail!("Can't deal with light")
    }
    if num_unk != 0 {
        bail!("Can't deal with num_unk")
    }
    if zero10 != 0 {
        bail!("Can't deal with unk10")
    }
    if zero2c != 0 {
        bail!("Can't deal with unk2c")
    }
    /*if unk30 != 0 {
        bail!("Can't deal with unk30")
    }*/
    if zero34 != 0 {
        bail!("Can't deal with unk34")
    }
    /*if unk38 != 0 {
        bail!("Can't deal with unk38")
    }*/
    if zero3c != 0 {
        bail!("Can't deal with unk3c")
    }

    // Note on offset_unk80
    //   This data appears to be 16 (potentially useful?) bytes followed by every byte value repeated x4,
    //   then 12 bytes of something

    file.seek_noop(offset_name)?;

    let num_bone_name = file.read_u32()?;
    let num_material_name = file.read_u32()?;
    let offset_bone_name = file.read_u64()?;
    let offset_material_name = file.read_u64()?;
    if num_bone_name != num_bones as u32 {
        bail!("num_bone_name");
    }
    // isn't always right. See chr042
    /*if num_material_name != num_material as u32 {
        bail!("num_material_name");
    }*/
    let mut bone_material_offset: Vec<u64> = Vec::new();
    for _ in 0..num_bones {
        bone_material_offset.push(file.read_u64()?)
    }
    let mut material_name_offset: Vec<u64> = Vec::new();
    for _ in 0..num_material_name {
        material_name_offset.push(file.read_u64()?)
    }

    file.seek_noop(offset_mesh)?;

    struct Attr {
        vtype: u8,
        normalize: u8,
        num: u16,
        dtype: u8,
        flags: u8,
        offset: u16,
    }

    #[derive(Default)]
    struct InMesh {
        offset_vertex: u64,
        offset_index: u64,
        offset_bone_map: u64,
        offset_unk: u64,
        offset_attr: u64,
        num_bone: u16,
        num_attr: u16,
        vertex_size: u32,
        num_vertex: u32,
        num_index: u32,
        name_offset: u64,

        vertexs: Vec<Vec<u8>>,
        bone_map: Vec<u32>,
        indexs: Vec<u16>,
        attrs: Vec<Attr>,

        name: String,
    }

    let mut meshs: Vec<InMesh> = Vec::new();
    for i in 0..num_mesh {
        let offset_vertex = file.read_u64()?;
        let offset_index = file.read_u64()?;

        let offset_bone_map = file.read_u64()?;
        let offset_unk = file.read_u64()?;

        let offset_attr = file.read_u64()?;
        let num_bone = file.read_u16()?;
        let num_attr = file.read_u16()?;
        let vertex_size = file.read_u32()?;

        let unk30 = file.read_u8()?; // 0, 2, 3, 4
        let unk31 = file.read_u8()?; // 5, 1
        println!("[{i}]{unk30}, {unk31}");
        let zero32 = file.read_u16()?; // 0
        let name_hash = file.read_u32()?;
        let name_offset = file.read_u64()?;

        let material_id = file.read_u32()?;
        let num_vertex = file.read_u32()?;
        let num_index = file.read_u32()?;
        let zero4c = file.read_u32()?; // 0

        let zero50 = file.read_u32()?; // 0
        let radius = file.read_f32()?;
        let center = file.read_f32vec3()?;
        let bound = file.read_f32vec3()?;

        let zero70 = file.read_u32()?;
        let zero74 = file.read_u32()?;
        let zero78 = file.read_u32()?;
        let zero7c = file.read_u32()?;

        if zero32 != 0 {
            bail!("Can't deal with unk32")
        }
        if zero4c != 0 {
            bail!("Can't deal with unk4c")
        }
        if zero50 != 0 {
            bail!("Can't deal with unk50")
        }
        if zero70 != 0 {
            bail!("Can't deal with unk70")
        }
        if zero74 != 0 {
            bail!("Can't deal with unk74")
        }
        if zero78 != 0 {
            bail!("Can't deal with unk78")
        }
        if zero7c != 0 {
            bail!("Can't deal with unk7c")
        }

        let mesh = InMesh {
            offset_vertex,
            offset_index,
            offset_bone_map,
            offset_unk,
            offset_attr,
            num_bone,
            num_attr,
            vertex_size,
            num_vertex,
            num_index,
            name_offset,
            ..InMesh::default()
        };

        meshs.push(mesh)
    }

    for (i, mesh) in meshs.iter_mut().enumerate() {
        file.seek_noop(mesh.offset_vertex)?;
        for _ in 0..mesh.num_vertex {
            let mut vertex = vec![0u8; mesh.vertex_size as usize];
            file.read_exact(&mut vertex)?;
            mesh.vertexs.push(vertex);
        }

        file.seek_noop(mesh.offset_bone_map)?;
        for _ in 0..mesh.num_bone {
            mesh.bone_map.push(file.read_u32()?);
        }

        file.seek_noop(mesh.offset_index)?;
        for _ in 0..mesh.num_index {
            mesh.indexs.push(file.read_u16()?);
        }

        file.seek_assert_align_up(mesh.offset_attr, 4)?;
        for _ in 0..mesh.num_attr {
            let vtype = file.read_u8()?;
            let normalize = file.read_u8()?;
            let num = file.read_u16()?;
            let dtype = file.read_u8()?;
            let flags = file.read_u8()?;
            let offset = file.read_u16()?;
            if normalize != 0 {
                bail!("Can't deal with normalize")
            }
            if flags != 0 {
                bail!("Can't deal with flags")
            }
            // println!("[{i}] -- {vtype} {normalize} {num} {dtype} {flags}");
            mesh.attrs.push(Attr {
                vtype,
                normalize,
                num,
                dtype,
                flags,
                offset,
            })
        }
    }

    file.seek_noop(offset_material)?;
    for _ in 0..num_material {
        let name_hash = file.read_u32()?;
        //...
    }

    file.seek(SeekFrom::Start(offset_inv_matrix))?;
    let mut inv_matrixs: Vec<Mat4x4> = Vec::new();
    for _ in 0..num_bones {
        inv_matrixs.push(file.read_f32m3x4()?);
    }

    file.seek(SeekFrom::Start(offset_tree))?;
    let magic = file.read_magic()?;
    let tree_bytes = file.read_u32()?;
    let tree_unk8 = file.read_u32()?;
    let tree_footer_size = file.read_u32()?;
    let tree_num_bones = file.read_u16()?;
    let tree_unk12_g = file.read_u16()?;
    let tree_unk14 = file.read_u32()?;
    if tree_num_bones != num_bones {
        bail!("num_bones = {num_bones:x}, tree_num_bones = {tree_num_bones:x}")
    }
    let offset_bone = file.tell()? + file.read_u32()? as u64;
    let offset_parents = file.tell()? + file.read_u32()? as u64;
    let offset_bone_name_hash = file.tell()? + file.read_u32()? as u64;
    let offset_e = file.tell()? + file.read_u32()? as u64;
    let offset_f = file.tell()? + file.read_u32()? as u64;
    let offset_g = file.tell()? + file.read_u32()? as u64;
    let tree_unk_a = file.read_u32()?;
    let tree_unk_b = file.read_u32()?;
    let tree_unk_c = file.read_u32()?;
    let num_rel = file.read_u32()?;

    let mut rels: Vec<(u16, u16)> = Vec::new();
    for _ in 0..num_rel {
        let child = file.read_u16()?;
        let parent = file.read_u16()?;
        rels.push((child, parent))
    }

    #[derive(Default)]
    struct Bone {
        rotation: Vec4,
        position: Vec4,
        scale: Vec4,
        parent: u16,
        children: Vec<u16>,
        name_hash: u32,
        name: String,
    }

    let mut bones: Vec<Bone> = Vec::new();
    file.seek_noop(offset_bone)?;
    for _ in 0..num_bones {
        let rotation = file.read_f32vec4()?;
        let position = file.read_f32vec4()?;
        let scale = file.read_f32vec4()?;
        bones.push(Bone {
            rotation,
            position,
            scale,
            ..Bone::default()
        })
    }

    let mut root_bones: Vec<u16> = Vec::new();
    file.seek_noop(offset_parents)?;
    for i in 0..num_bones {
        let index = file.read_u16()? as usize;
        let parent = rels[index].1 & 0x7FFF; // What is the high bit? see chr359
        if index >= rels.len() || rels[index].0 != i {
            bail!("Bone child {i}")
        }
        bones[i as usize].parent = parent;
        if parent != 0x7FFF {
            bones[parent as usize].children.push(i);
        } else {
            root_bones.push(i)
        }
    }

    file.seek_assert_align_up(offset_g, 4)?;

    // something in between..
    file.seek(SeekFrom::Start(offset_bone_name_hash))?; // should align to 16
    for bone in &mut bones {
        bone.name_hash = file.read_u32()?;
    }

    file.seek_noop(offset_e)?;

    for (i, bone) in bones.iter_mut().enumerate() {
        file.seek(SeekFrom::Start(
            offset_string_table + bone_material_offset[i],
        ))?;
        bone.name = file.read_u8str()?;
    }
    for mesh in &mut meshs {
        file.seek(SeekFrom::Start(offset_string_table + mesh.name_offset))?;
        mesh.name = file.read_u8str()?;
    }

    ///////////////////////////////////////////////////////////////////////////////////

    let mut controllers: Vec<Controller> = Vec::new();
    let mut geometries: Vec<Geometry> = Vec::new();
    for (i, mesh) in meshs.iter().enumerate() {
        let mut texcoord_used: Vec<u8> = mesh
            .attrs
            .iter()
            .map(|a| a.vtype)
            .filter(|&v| v == 5 || v == 6 || v == 7)
            .collect();
        texcoord_used.sort_by_key(|&v| match v {
            5 => 2,
            6 => 0,
            7 => 1,
            _ => 4,
        });

        let mut bone_array: Vec<u8> = Vec::new();
        let mut weight_array: Vec<f32> = Vec::new();
        let mut bone_attr_num = 0;
        let mut weight_attr_num = 0;
        let mut vertex_sources: Vec<Source> = Vec::new();
        let mut vertex_inputs: Vec<Input> = Vec::new();
        let mut primitive_inputs: Vec<SharedInput> = vec![SharedInput {
            semantic: "VERTEX".to_owned(),
            source: format!("#mesh{i}-vertices"),
            offset: 0,
            set: None,
        }];
        for (attr_i, attr) in mesh.attrs.iter().enumerate() {
            let num = if attr.vtype == 1 && attr.num > 3 {
                3
            } else {
                attr.num
            };
            let data: Vec<f32> = match attr.dtype {
                0 => {
                    let data = mesh
                        .vertexs
                        .iter()
                        .flat_map(|v| &v[attr.offset as usize..][..num as usize])
                        .copied();
                    if attr.vtype == 10 {
                        bone_array = data.collect();
                        bone_attr_num = num;
                        continue;
                    }
                    data.map(|v| v as f32 / 255.0).collect()
                }
                8 => mesh
                    .vertexs
                    .iter()
                    .flat_map(|v| {
                        (0..num).map(|j| {
                            half::f16::from_le_bytes(
                                v[(attr.offset + j * 2) as usize..][..2].try_into().unwrap(),
                            )
                            .to_f32()
                        })
                    })
                    .collect(),
                9 => mesh
                    .vertexs
                    .iter()
                    .flat_map(|v| {
                        (0..num).map(|j| {
                            f32::from_le_bytes(
                                v[(attr.offset + j * 4) as usize..][..4].try_into().unwrap(),
                            )
                        })
                    })
                    .collect(),
                _ => bail!("Unknown dtype {}", attr.dtype),
            };

            if attr.vtype == 11 {
                weight_attr_num = num;
                weight_array = data;
                continue;
            }

            if num > 4 {
                bail!("Attribute too large");
            }
            let params = match attr.vtype {
                1 | 2 | 3 => &["X", "Y", "Z", "W"][0..num as usize],
                5 | 6 | 7 => &["S", "T", "U", "V"][0..num as usize],
                9 => &["R", "G", "B", "A"][0..num as usize],
                _ => bail!("Unknown vtype {}", attr.vtype),
            };
            let source_id = format!("mesh{i}-attr{attr_i}");
            let array_id = format!("{source_id}-array");
            vertex_sources.push(Source {
                id: source_id.clone(),
                array_element: ArrayElement::FloatArray {
                    id: array_id.clone(),
                    array: data,
                },
                technique_common: TechniqueCommon {
                    elements: vec![TechniqueCommonElement::Accessor {
                        count: mesh.num_vertex,
                        source: format!("#{array_id}"),
                        stride: num as u32,
                        params: params
                            .iter()
                            .map(|&s| Param {
                                name: s.to_owned(),
                                type_: "float".to_owned(),
                            })
                            .collect(),
                    }],
                },
            });
            match attr.vtype {
                1 => {
                    vertex_inputs.push(Input {
                        semantic: "POSITION".to_owned(),
                        source: format!("#{source_id}"),
                    });
                }
                2 => primitive_inputs.push(SharedInput {
                    semantic: "NORMAL".to_owned(),
                    source: format!("#{source_id}"),
                    offset: 0,
                    set: None,
                }),
                3 => primitive_inputs.push(SharedInput {
                    semantic: "TANGENT".to_owned(),
                    source: format!("#{source_id}"),
                    offset: 0,
                    set: None,
                }),
                5 | 6 | 7 => primitive_inputs.push(SharedInput {
                    semantic: "TEXCOORD".to_owned(),
                    source: format!("#{source_id}"),
                    offset: 0,
                    set: Some(
                        texcoord_used
                            .iter()
                            .enumerate()
                            .find(|(_, vtype)| **vtype == attr.vtype)
                            .unwrap()
                            .0 as u32,
                    ),
                }),
                9 => primitive_inputs.push(SharedInput {
                    semantic: "COLOR".to_owned(),
                    source: format!("#{source_id}"),
                    offset: 0,
                    set: None,
                }),

                _ => bail!("Unknown vtype {}", attr.vtype),
            }
        }

        let mut unfold_indices: Vec<u16> = vec![];
        for (k, window) in mesh.indexs.windows(3).enumerate() {
            if k % 2 == 0 {
                unfold_indices.extend_from_slice(window)
            } else {
                unfold_indices.push(window[1]);
                unfold_indices.push(window[0]);
                unfold_indices.push(window[2]);
            }
        }

        geometries.push(Geometry {
            id: format!("mesh{i}"),
            geometric_element: GeometricElement::Mesh {
                sources: vertex_sources,
                vertices: Vertices {
                    id: format!("mesh{i}-vertices"),
                    inputs: vertex_inputs,
                },
                primitive_elements: vec![PrimitiveElements::Triangles {
                    count: unfold_indices.len() as u32 / 3,
                    inputs: primitive_inputs,
                    p: unfold_indices,
                }],
            },
        });

        if weight_attr_num != bone_attr_num {
            bail!("Mismatched bone num");
        }

        #[allow(unused_assignments)]
        if weight_attr_num == 0 {
            weight_attr_num = 1;
            bone_attr_num = 1;
            weight_array = std::iter::repeat(1.0)
                .take(mesh.num_vertex as usize)
                .collect();
            bone_array = std::iter::repeat(0)
                .take(mesh.num_vertex as usize)
                .collect();
        }

        controllers.push(Controller {
            id: format!("controller{i}"),
            skin: Skin {
                source: format!("#mesh{i}"),
                sources: vec![
                    Source {
                        id: format!("controller{i}-joint"),
                        array_element: ArrayElement::NameArray {
                            id: format!("controller{i}-joint-array"),
                            array: mesh.bone_map.iter().map(|&j| format!("bone{j}")).collect(),
                        },
                        technique_common: TechniqueCommon {
                            elements: vec![TechniqueCommonElement::Accessor {
                                count: mesh.bone_map.len() as u32,
                                source: format!("#controller{i}-joint-array"),
                                stride: 1,
                                params: vec![Param {
                                    name: "JOINT".to_owned(),
                                    type_: "name".to_owned(),
                                }],
                            }],
                        },
                    },
                    Source {
                        id: format!("controller{i}-inv"),
                        array_element: ArrayElement::FloatArray {
                            id: format!("controller{i}-inv-array"),
                            array: mesh
                                .bone_map
                                .iter()
                                .flat_map(|&j| {
                                    let m = inv_matrixs[j as usize];
                                    m.row_iter().flatten().copied().collect::<Vec<_>>()
                                })
                                .collect(),
                        },
                        technique_common: TechniqueCommon {
                            elements: vec![TechniqueCommonElement::Accessor {
                                count: mesh.bone_map.len() as u32,
                                source: format!("#controller{i}-inv-array"),
                                stride: 16,
                                params: vec![Param {
                                    name: "TRANSFORM".to_owned(),
                                    type_: "float4x4".to_owned(),
                                }],
                            }],
                        },
                    },
                    Source {
                        id: format!("controller{i}-weight"),
                        array_element: ArrayElement::FloatArray {
                            id: format!("controller{i}-weight-array"),
                            array: weight_array,
                        },
                        technique_common: TechniqueCommon {
                            elements: vec![TechniqueCommonElement::Accessor {
                                count: mesh.num_vertex * bone_attr_num as u32,
                                source: format!("#controller{i}-weight-array"),
                                stride: 1,
                                params: vec![Param {
                                    name: "WEIGHT".to_owned(),
                                    type_: "float".to_owned(),
                                }],
                            }],
                        },
                    },
                ],
                joints: Joints {
                    inputs: vec![
                        Input {
                            semantic: "JOINT".to_owned(),
                            source: format!("#controller{i}-joint"),
                        },
                        Input {
                            semantic: "INV_BIND_MATRIX".to_owned(),
                            source: format!("#controller{i}-inv"),
                        },
                    ],
                },
                vertex_weights: VertexWeights {
                    count: mesh.num_vertex,
                    inputs: vec![
                        SharedInput {
                            semantic: "JOINT".to_owned(),
                            source: format!("#controller{i}-joint"),
                            offset: 0,
                            set: None,
                        },
                        SharedInput {
                            semantic: "WEIGHT".to_owned(),
                            source: format!("#controller{i}-weight"),
                            offset: 1,
                            set: None,
                        },
                    ],
                    vcount: std::iter::repeat(bone_attr_num as u8)
                        .take(mesh.num_vertex as usize)
                        .collect(),
                    v: bone_array
                        .iter()
                        .enumerate()
                        .flat_map(|(j, &b)| [b as u32, j as u32])
                        .collect(),
                },
            },
        })
    }

    fn create_node(bones: &[Bone], index: u16) -> Node {
        let bone = &bones[index as usize];
        let pos = translation(&bone.position.xyz());
        let rotation = quat_to_mat4(&Quat::from_vector(bone.rotation));
        let scale = scaling(&bone.scale.xyz());
        let m = pos * rotation * scale;

        Node {
            id: format!("bone{index}"),
            name: format!("bone{}-{}", index, bone.name),
            type_: NodeType::Joint,
            matrix: Some(m),
            instance_controllers: vec![],
            instance_geometries: vec![],
            nodes: bone
                .children
                .iter()
                .map(|i| create_node(bones, *i))
                .collect(),
        }
    }

    let bone_root = Node {
        id: format!("boneroot"),
        name: format!("boneroot"),
        type_: NodeType::Joint,
        matrix: Some(identity()),
        instance_controllers: vec![],
        instance_geometries: vec![],
        nodes: root_bones.iter().map(|i| create_node(&bones, *i)).collect(),
    };

    let mut nodes = vec![bone_root];

    for (i, mesh) in meshs.iter().enumerate() {
        nodes.push(Node {
            id: format!("mesh{i}-node"),
            name: format!("mesh{}-{}", i, mesh.name),
            type_: NodeType::Node,
            matrix: None,
            instance_controllers: vec![InstanceController {
                url: format!("#controller{i}"),
                skeletons: vec!["#boneroot".to_owned()],
            }],
            instance_geometries: vec![],
            nodes: vec![],
        })
    }

    let visual_scene = VisualScene {
        id: format!("scene"),
        nodes,
    };

    let dae = Collada {
        asset: Asset {
            created: "2022-06-19T15:05:15".to_owned(),
            modified: "2022-06-19T15:05:15".to_owned(),
        },
        libraries: vec![
            Library::Geometries { geometries },
            Library::VisualScenes {
                visual_scenes: vec![visual_scene],
            },
            Library::Controllers { controllers },
        ],
        scene: Scene {
            instance_visual_scene: "#scene".to_owned(),
        },
    };

    if args.len() > 2 {
        dae.save(Path::new(&args[2]))?;
    }

    Ok(())
}
