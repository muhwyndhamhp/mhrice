use once_cell::sync::Lazy;
use std::collections::HashMap;

// The suffix list for a particular file format is ordered that the first version comes first
pub static SUFFIX_MAP: Lazy<HashMap<&'static str, Vec<u32>>> = Lazy::new(|| {
    // Initial version list dumped from 3.6.1.0~3.9.1.0 PC
    let mut hash_map = HashMap::from_iter([
        ("aimap", vec![0x28]),
        ("aimapattr", vec![0x29]),
        ("ainvm", vec![8]),
        ("ainvmmgr", vec![2]),
        ("aivspc", vec![4]),
        ("aivspcmgr", vec![2]),
        ("aiwayp", vec![3]),
        ("areamap", vec![8]),
        ("areaquery", vec![2]),
        ("auto", vec![4]),
        ("bhvt", vec![0x28]),
        ("bnk", vec![2]),
        ("ccbk", vec![1]),
        ("cdef", vec![3]),
        ("cfil", vec![7]),
        ("chain", vec![0x23]),
        ("clip", vec![0x28]),
        ("clsm", vec![0xf]),
        ("cmat", vec![3]),
        ("cset", vec![1]),
        ("dblc", vec![1]),
        ("dlg", vec![2]),
        ("dlglist", vec![0]),
        ("dlgtml", vec![40000]),
        ("dlgtmllist", vec![40000000]),
        ("eem", vec![0]),
        ("efx", vec![0x280223]),
        ("fbik", vec![6]),
        ("fbxskel", vec![3]),
        ("fgrl", vec![1]),
        ("filter", vec![1]),
        ("fol", vec![0]),
        ("fslt", vec![2]),
        ("fsm", vec![0x10]),
        ("fsmv2", vec![0x28]),
        ("fxct", vec![4]),
        ("gcf", vec![0x18]),
        ("gcp", vec![1]),
        ("gp", vec![0]),
        ("gpbf", vec![1]),
        ("gpumotlist", vec![0x1c9]),
        ("gsty", vec![1]),
        ("gui", vec![0x61a96]),
        ("guisd", vec![1]),
        ("hapvib", vec![0x6bb788fe]),
        ("ies", vec![2]),
        ("ift", vec![4]),
        ("ikbodyrig", vec![1]),
        ("ikdamage", vec![4]),
        ("ikleg2", vec![0xc]),
        ("iklookat", vec![2]),
        ("iklookat2", vec![8]),
        ("ikls", vec![5]),
        ("ikspinecg", vec![1]),
        ("iktrain", vec![4]),
        ("jcns", vec![0xe]),
        ("jmap", vec![0x10]),
        ("jointlodgroup", vec![2]),
        ("layergrid", vec![7]),
        ("lfa", vec![4]),
        ("lform", vec![7]),
        ("lmap", vec![2]),
        ("lod", vec![3]),
        ("lprb", vec![6]),
        ("mcambank", vec![3]),
        ("mcamlist", vec![0x11]),
        ("mcol", vec![0x2723]),
        ("mdf2", vec![0x13]),
        ("mesh", vec![0x77b089b0]),
        ("mmtr", vec![0x7db96331]),
        ("mmtrs", vec![0x7db96331]),
        ("mot", vec![0x1c8]),
        ("motbank", vec![3]),
        ("motblend", vec![0x1c8]),
        ("motcam", vec![8]),
        ("motfsm", vec![0x11]),
        ("motfsm2", vec![0x2a]),
        ("motlist", vec![0x1e4]),
        ("mottree", vec![10]),
        ("mov", vec![1]),
        ("msg", vec![0x11]),
        ("nar", vec![1]),
        ("ncf", vec![9]),
        ("nmr", vec![0x11]),
        ("nnfp", vec![1]),
        ("oft", vec![1]),
        ("path", vec![0]),
        ("pci", vec![2]),
        ("pck", vec![3]),
        ("pfb", vec![0x11]),
        ("prb", vec![9]),
        ("psop", vec![1]),
        ("pup", vec![0]),
        ("rbsl", vec![1]),
        ("rcfg", vec![3]),
        ("rcol", vec![0x12]),
        ("retarget", vec![5]),
        ("retargetrig", vec![4]),
        ("rfl", vec![1]),
        ("rtbs", vec![4]),
        ("rtex", vec![5]),
        ("rtmr", vec![3]),
        ("scn", vec![0x14]),
        ("scns", vec![1]),
        ("sdf", vec![0x7db96331]),
        ("skeleton", vec![3]),
        ("spf", vec![0x6be4bb9a]),
        ("spt", vec![0x6bf3afba]),
        ("sss", vec![2]),
        ("sst", vec![5]),
        ("stmesh", vec![2]),
        ("svgn", vec![3]),
        ("svgsq", vec![1]),
        ("tean", vec![1]),
        ("terr", vec![0x2718]),
        ("tex", vec![0x1c]),
        ("tml", vec![0x9c41]),
        ("tmlbld", vec![0x9c44]),
        ("tmlfsm2", vec![0x262f641]),
        ("trtd", vec![0xbbc]),
        ("ucurve", vec![0x28]),
        ("ucurvelist", vec![0x28]),
        ("user", vec![2]),
        ("uvar", vec![2]),
        ("uvs", vec![7]),
        ("vsdf", vec![0x7db96332]),
        ("wcb", vec![1]),
        ("wcbk", vec![3]),
        ("wcc", vec![2]),
        ("wcd", vec![1]),
        ("wcfe", vec![2]),
        ("wcggp", vec![1]),
        ("wcgp", vec![5]),
        ("wcja", vec![3]),
        ("wcjm", vec![1]),
        ("wcjmv", vec![2]),
        ("wcjr", vec![2]),
        ("wcmo", vec![3]),
        ("wcms", vec![2]),
        ("wcmsw", vec![4]),
        ("wcmts", vec![2]),
        ("wcp", vec![3]),
        ("wcr", vec![1]),
        ("wcrb", vec![5]),
        ("wcrd", vec![5]),
        ("wcsa", vec![2]),
        ("wcsf", vec![1]),
        ("wcsgp", vec![1]),
        ("wcss", vec![2]),
        ("wcst", vec![5]),
        ("wcsw", vec![5]),
        ("wcswn", vec![2]),
        ("wct", vec![2]),
        ("wcta", vec![3]),
        ("wcv", vec![3]),
        ("wel", vec![0xb]),
        ("wem", vec![2]),
        ("wfa", vec![2]),
        ("wfc", vec![1]),
        ("wgs", vec![2]),
        ("wid", vec![2]),
        ("wlqg", vec![2]),
        ("wms", vec![5]),
        ("wpi", vec![3]),
        ("wss", vec![2]),
        ("wtos", vec![1]),
        ("wtot", vec![3]),
    ]);

    // Dumped from sunbreak demo
    let sb_demo = [
        ("aimap", 0x29),
        ("aimapattr", 0x2a),
        ("ainvm", 0xe),
        ("ainvmmgr", 3),
        ("aivspc", 5),
        ("aivspcmgr", 3),
        ("aiwayp", 4),
        ("aiwaypmgr", 3),
        ("areamap", 8),
        ("areaquery", 2),
        ("auto", 4),
        ("bhvt", 0x29),
        ("bnk", 2),
        ("caphand", 0),
        ("ccbk", 3),
        ("cdef", 4),
        ("cfil", 7),
        ("chain", 0x30),
        ("chainwnd", 0),
        ("chf", 2),
        ("clip", 0x2b),
        ("clsm", 0xf),
        ("cmat", 3),
        ("cset", 3),
        ("dblc", 1),
        ("dlg", 5),
        ("dlglist", 3),
        ("dlgtml", 43000),
        ("dlgtmllist", 43000000),
        ("eem", 0),
        ("efcsv", 1),
        ("efx", 0x2b02aa),
        ("fbik", 6),
        ("fbxskel", 4),
        ("fgrl", 1),
        ("filter", 1),
        ("fpolygon", 1),
        ("fslt", 2),
        ("fsm", 0x11),
        ("fsmv2", 0x29),
        ("fxct", 4),
        ("gcf", 0x19),
        ("gcp", 1),
        ("gpbf", 2),
        ("gpumotlist", 0x1f0),
        ("gsty", 2),
        ("gui", 0x68fd0),
        ("guisd", 1),
        ("hapvib", 0x6bb788fe),
        ("ies", 2),
        ("ift", 4),
        ("ikbodyrig", 3),
        ("ikdamage", 4),
        ("ikhd", 0),
        ("ikleg2", 0x11),
        ("iklookat", 2),
        ("iklookat2", 0xe),
        ("ikls", 0xd),
        ("ikmulti", 2),
        ("ikspinecg", 1),
        ("iktrain", 4),
        ("jcns", 0x15),
        ("jmap", 0x13),
        ("jointlodgroup", 2),
        ("layergrid", 7),
        ("lfa", 4),
        ("lmap", 4),
        ("lod", 3),
        ("lprb", 6),
        ("mcambank", 3),
        ("mcamlist", 0x13),
        ("mcol", 0x32dc),
        ("mdf2", 0x17),
        ("mesh", 0x7db70c80),
        ("mmtr", 0xd205294),
        ("mmtrs", 0xd205294),
        ("mot", 0x1ef),
        ("motbank", 3),
        ("motblend", 0x1ef),
        ("motcam", 9),
        ("motface", 0xf),
        ("motfsm", 0x12),
        ("motfsm2", 0x2b),
        ("motlist", 0x210),
        ("mottree", 0xe),
        ("mov", 1),
        ("mpci", 1),
        ("msg", 0x11),
        ("nar", 1),
        ("ncf", 9),
        ("nmr", 0x11),
        ("nnfp", 1),
        ("oft", 1),
        ("pci", 3),
        ("pck", 3),
        ("pfb", 0x11),
        ("pfnn", 0),
        ("prb", 9),
        ("prvs", 1),
        ("psop", 1),
        ("pup", 0),
        ("rbsl", 1),
        ("rcfg", 4),
        ("rcol", 0x14),
        ("refskel", 4),
        ("retargetfleg", 0),
        ("retargetrig", 8),
        ("rfl", 1),
        ("rtbs", 4),
        ("rtex", 5),
        ("rtmr", 5),
        ("scb", 1),
        ("scl", 1),
        ("scn", 0x14),
        ("scns", 1),
        ("sdf", 0xd205294),
        ("sdftex", 3),
        ("skeleton", 4),
        ("sss", 3),
        ("sst", 8),
        ("star", 3),
        ("stl", 3),
        ("strands", 3),
        ("sts", 1),
        ("svgn", 3),
        ("svgsq", 1),
        ("tean", 1),
        ("terr", 0x32d0),
        ("tex", 0x22),
        ("tml", 0xa7f9),
        ("tmlbld", 0xa803),
        ("tmlfsm2", 0x2724439),
        ("trtd", 0xbbc),
        ("ucurve", 0x2b),
        ("ucurvelist", 0x2b),
        ("user", 2),
        ("uvar", 3),
        ("uvs", 7),
        ("vsdf", 0xd205295),
        ("wcb", 1),
        ("wcbk", 3),
        ("wcc", 2),
        ("wcd", 1),
        ("wcfe", 2),
        ("wcggp", 1),
        ("wcgp", 5),
        ("wcja", 3),
        ("wcjm", 1),
        ("wcjmv", 2),
        ("wcjr", 2),
        ("wcmo", 3),
        ("wcms", 2),
        ("wcmsw", 4),
        ("wcmts", 2),
        ("wcp", 3),
        ("wcr", 1),
        ("wcrb", 5),
        ("wcrd", 5),
        ("wcsa", 2),
        ("wcsf", 1),
        ("wcsgp", 1),
        ("wcss", 2),
        ("wcst", 5),
        ("wcsw", 5),
        ("wcswn", 2),
        ("wct", 2),
        ("wcta", 3),
        ("wcv", 3),
        ("wel", 0xb),
        ("wem", 2),
        ("wfa", 2),
        ("wfc", 1),
        ("wgs", 2),
        ("wid", 2),
        ("wlqg", 2),
        ("wms", 5),
        ("wpi", 3),
        ("wss", 2),
        ("wtos", 1),
        ("wtot", 3),
    ];

    for (name, version) in sb_demo {
        let entry = hash_map.entry(name).or_default();
        if entry.last() != Some(&version) {
            entry.push(version);
        }
    }

    hash_map
});
