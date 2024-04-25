use std::collections::HashMap;

use rustdoc_types::{Crate, GenericArg, GenericArgs, Id, ItemEnum, Type};

fn main() {
    let res: Crate = serde_json::from_str(include_str!("../spacegame_prototype.json")).unwrap();

    let mut ids_impl_component = Vec::new();
    let mut ids_impl_resource = Vec::new();

    let mut systems_querying_components: HashMap<Id, Vec<(Id, bool)>> = HashMap::new();

    for i in res.index.values() {
        if let ItemEnum::Impl(imp) = &i.inner {
            if let Some(trt) = &imp.trait_ {
                if let Type::ResolvedPath(path) = &imp.for_ {
                    if trt.name == "Component" {
                        ids_impl_component.push(path.id.clone());
                        continue;
                    } else if trt.name == "Resource" {
                        ids_impl_resource.push(path.id.clone());
                        continue;
                    }
                }
            }
        }

        // if i.visibility == rustdoc_types::Visibility::Default {
        //     continue;
        // }

        // if i.crate_id == 0 {
        //     if let (Some(name), Some(span)) = (&i.name, &i.span) {
        //         let inn = match i.inner {
        //             rustdoc_types::ItemEnum::Struct(_) => "Struct",
        //             rustdoc_types::ItemEnum::Enum(_) => "Enum",
        //             rustdoc_types::ItemEnum::Function(_) => "Function",
        //             rustdoc_types::ItemEnum::Constant(_) => "Constant",
        //             _ => continue,
        //         };
        //         count += 1;
        //         println!("{} {} {}", name, inn, span.filename.to_str().unwrap());

        //         // println!("{:?}", i.visibility);
        //     }
        // }
    }

    let is_component = |id: &Id| -> bool { ids_impl_component.iter().any(|cid| cid == id) };

    for i in res.index.values() {
        // Our crate
        if i.crate_id == 0 {
            if let (Some(name), Some(span), ItemEnum::Function(func)) = (&i.name, &i.span, &i.inner)
            {
                for (_n, ty) in func.decl.inputs.iter() {
                    if let Type::ResolvedPath(path) = ty {
                        if path.name == "Query" {
                            if let Some(ga) = &path.args {
                                if let GenericArgs::AngleBracketed { args, .. } = ga.as_ref() {
                                    for a in args.iter() {
                                        if let GenericArg::Type(Type::BorrowedRef {
                                            type_,
                                            mutable,
                                            ..
                                        }) = a
                                        {
                                            if let Type::ResolvedPath(path) = type_.as_ref() {
                                                if is_component(&path.id) {
                                                    systems_querying_components
                                                        .entry(i.id.clone())
                                                        .or_default()
                                                        .push((path.id.clone(), *mutable));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                // println!("{} {} {}", name, inn, span.filename.to_str().unwrap());

                // println!("{:?}", i.visibility);
            }
        }
    }

    let get_path = |id: &Id| -> String { res.paths.get(id).unwrap().path.join("::") };

    println!();
    println!();

    println!("Components ({})", ids_impl_component.len());
    for id in ids_impl_component.iter() {
        let path = get_path(id);
        println!("  {}", path);

        for (sys_id, comp_ids) in systems_querying_components.iter() {
            if let Some((_, m)) = comp_ids.iter().find(|(cid, m)| cid == id) {
                println!(
                    "    Queried {} in {}",
                    match *m {
                        true => "Mutably",
                        false => "Immutably",
                    },
                    get_path(sys_id)
                );
            }
        }

        // if let Some(x) = res.index.get(id) {

        // }
    }

    println!();

    println!("Resources ({})", ids_impl_resource.len());
    for id in ids_impl_resource.iter() {
        let path = get_path(id);
        println!("  {}", path);
        // if let Some(x) = res.index.get(id) {
        //     println!("{:?} Component", x.name);
        // }
    }

    // println!("{:#?}", systems_querying_components);
}
