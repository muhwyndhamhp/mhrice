let g_cookie_consent = false;
let g_language_code = "en";

let g_navbar_menu_active = false;

let g_classes_to_hide = new Set();

let g_cur_map_explain = "default";

let g_map_scale = 100;
let g_map_layer = 0;
let g_cur_map_filter = "all";

let g_cur_item_filter = "all";

let g_toc = null;

window.onload = function () {
    check_cookie();
    switchLanguage();
    hide_class("mh-ride-cond");
    hide_class("mh-invalid-meat");
    hide_class("mh-invalid-part");
    hide_class("mh-no-preset");

    change_sort("monster", 1);
    change_sort("item", 1);
    change_sort("armor", 1);
}

function check_cookie() {
    const cookies = document.cookie.split(";");
    for (const cookie of cookies) {
        const s = cookie.trim().split("=");
        const cookie_name = s[0];
        const cookie_value = s[1];
        if (cookie_name === null || cookie_value === null) {
            continue;
        }
        if (cookie_name === "consent" && cookie_value === "yes") {
            g_cookie_consent = true;
        }

        if (cookie_name === "mh-language") {
            g_language_code = cookie_value
            if (!(supported_mh_lang.includes(g_language_code))) {
                g_language_code = "en";
            }
        }
    }

    if (g_cookie_consent) {
        document.getElementById("cookie-yes").checked = true;
    } else {
        document.getElementById("cookie-no").checked = true;
    }
}

function enableCookie() {
    document.cookie = "consent=yes; path=/";
    g_cookie_consent = true;
}

function disableCookie() {
    g_cookie_consent = false;
    delete_all_cookie();
}

function delete_all_cookie() {
    const cookies = document.cookie.split(";");
    for (const cookie of cookies) {
        document.cookie = cookie.trim().split("=")[0] + "=;expires=Thu, 01 Jan 1970 00:00:00 GMT; path=/";
    }
}

function parse_sort_tag(node) {
    const tag = node.dataset.sort;
    return tag.split(',').map(n => parseInt(n))
}

function onChangeSort(select) {
    change_sort(select.id.slice(7 /*scombo-*/), parseInt(select.value))
}

function change_sort(list_name, selecter) {
    const ul = document.getElementById("slist-" + list_name);
    if (ul) {
        const new_ul = ul.cloneNode(false);

        const l = [];
        for (const e of ul.childNodes) {
            l.push(e);
        }

        l.sort(function (a, b) {
            const anode = parse_sort_tag(a);
            const bnode = parse_sort_tag(b);
            if (anode[selecter] === bnode[selecter]) {
                return anode[0] - bnode[0];
            } else {
                return anode[selecter] - bnode[selecter];
            }
        });

        for (const e of l) {
            new_ul.appendChild(e);
        }

        ul.parentNode.replaceChild(new_ul, ul);
    }
    const select = document.getElementById("scombo-" + list_name);
    if (select) {
        select.value = selecter
    }
}

function refresh_visibility(c) {
    for (const element of document.getElementsByClassName(c)) {
        let matched = false;
        for (const c of g_classes_to_hide) {
            if (element.classList.contains(c)) {
                matched = true;
                break;
            }
        }

        if (matched) {
            element.classList.add("mh-hidden");
        } else {
            element.classList.remove("mh-hidden");
        }
    }
}

function hide_class(c) {
    g_classes_to_hide.add(c);
    refresh_visibility(c);
}

function show_class(c) {
    g_classes_to_hide.delete(c);
    refresh_visibility(c);
}

function selectLanguage(language) {
    g_toc = null;
    g_language_code = language;
    switchLanguage();
    if (g_cookie_consent) {
        document.cookie = "mh-language=" + g_language_code + "; path=/";
    }
}

function switchLanguage() {
    document.getElementById("mh-lang-style").innerHTML =
        ".mh-lang:not([lang=\"" + g_language_code + "\"]) { display:none; }";

    for (const l of supported_mh_lang) {
        const menu_option = document.getElementById("mh-lang-menu-" + l);
        if (menu_option) {
            if (l === g_language_code) {
                menu_option.classList.add("has-text-weight-bold");
            } else {
                menu_option.classList.remove("has-text-weight-bold");
            }
        }
    }
}

function onCheckDisplay(checkbox, class_to_show, class_to_hide) {
    if (checkbox.checked) {
        show_class(class_to_show)
        if (class_to_hide != null) {
            hide_class(class_to_hide)
        }
    } else {
        hide_class(class_to_show)
        if (class_to_hide != null) {
            show_class(class_to_hide)
        }
    }
}

function onToggleNavbarMenu() {
    g_navbar_menu_active = !g_navbar_menu_active;
    if (g_navbar_menu_active) {
        document.getElementById("navbarBurger").classList.add("is-active");
        document.getElementById("navbarMenu").classList.add("is-active");
    } else {
        document.getElementById("navbarBurger").classList.remove("is-active");
        document.getElementById("navbarMenu").classList.remove("is-active");
    }
}

function onShowMapExplain(id) {
    if (g_cur_map_explain !== null) {
        document.getElementById("mh-map-explain-" + g_cur_map_explain).classList.add("mh-hidden");
        if (g_cur_map_explain !== "default") {
            document.getElementById("mh-map-icon-" + g_cur_map_explain).classList.remove("mh-map-select");
        }
    }
    g_cur_map_explain = id;
    document.getElementById("mh-map-explain-" + g_cur_map_explain).classList.remove("mh-hidden");
    document.getElementById("mh-map-icon-" + g_cur_map_explain).classList.add("mh-map-select");
}

function updateMapScale() {
    const map = document.getElementById("mh-map");
    map.style.width = g_map_scale + "%";
    map.style.paddingTop = g_map_scale + "%";
}

function scaleUpMap() {
    if (g_map_scale >= 500) {
        return
    }

    g_map_scale += 50;

    document.getElementById("button-scale-down").disabled = false;
    if (g_map_scale >= 500) {
        document.getElementById("button-scale-up").disabled = true;
    }

    updateMapScale()
}

function scaleDownMap() {
    if (g_map_scale <= 100) {
        return
    }

    g_map_scale -= 50

    document.getElementById("button-scale-up").disabled = false;
    if (g_map_scale <= 100) {
        document.getElementById("button-scale-down").disabled = true;
    }

    updateMapScale()
}

function switchMapLayer() {
    const prev = document.getElementById("mh-map-layer-" + g_map_layer);
    g_map_layer += 1;
    let cur = document.getElementById("mh-map-layer-" + g_map_layer);
    if (cur === null) {
        g_map_layer = 0;
        cur = document.getElementById("mh-map-layer-" + g_map_layer);
    }
    prev.classList.add("mh-hidden");
    cur.classList.remove("mh-hidden");
}

function changeMapFilter(filter) {
    const style = document.getElementById("mh-map-pop-style");
    if (style) {
        if (filter == "all") {
            style.innerHTML = "";
        } else {
            style.innerHTML =
                ".mh-map-pop:not([data-filter*=\""
                + filter + "\"]) { display:none; }";
        }
    }

    const filter_button_prefix = "mh-map-filter-";
    const prev = document.getElementById(filter_button_prefix + g_cur_map_filter);
    if (prev !== null) {
        prev.classList.remove("is-primary")
    }

    g_cur_map_filter = filter;

    const cur = document.getElementById(filter_button_prefix + g_cur_map_filter);
    if (cur !== null) {
        cur.classList.add("is-primary")
    }
}

function changeItemFilter(filter) {
    const style = document.getElementById("mh-item-list-style");
    if (style) {
        if (filter == "all") {
            style.innerHTML = "";
        } else {
            style.innerHTML =
                ".mh-item-filter-item:not([data-filter=\""
                + filter + "\"]) { display:none; }";
        }
    }

    const filter_button_prefix = "mh-item-filter-button-";
    const prev = document.getElementById(filter_button_prefix + g_cur_map_filter);
    if (prev !== null) {
        prev.classList.remove("is-primary")
    }

    g_cur_map_filter = filter;

    const cur = document.getElementById(filter_button_prefix + g_cur_map_filter);
    if (cur !== null) {
        cur.classList.add("is-primary")
    }
}

function doSearch() {
    const text = document.getElementById("mh-search").value.trim();
    if (text.length === 0) {
        return;
    }
    const matchers = text.split(' ').filter(m => m.length > 0);

    const results = [];
    for (const entry of g_toc) {
        let matched = 0;
        let matched_length = 0;
        for (const matcher of matchers) {
            if (entry.title.toLowerCase().includes(matcher.toLowerCase())) {
                matched += 1;
                matched_length += matcher.length;
            }
        }
        if (matched === 0) {
            continue;
        }

        const score = matched * 10 - (entry.title.length - matched_length);
        const result = { score, ...entry };
        results.push(result);
    }

    results.sort((a, b) => b.score - a.score);

    const ul = document.getElementById("mh-search-result");
    ul.replaceChildren();

    for (const result of results) {
        const link = document.createElement("a")
        link.setAttribute("href", result.path);
        link.appendChild(document.createTextNode(result.title));

        let tag = "";
        if (result.path.includes("monster")) {
            tag = "Monster";
        } else if (result.path.includes("armor")) {
            tag = "Armor";
        } else if (result.path.includes("skill")) {
            tag = "Skill";
        } else if (result.path.includes("item")) {
            tag = "Item";
        } else if (result.path.includes("map")) {
            tag = "Map";
        } else if (result.path.includes("quest")) {
            tag = "Quest";
        } else if (result.path.includes("weapon")) {
            tag = "Weapon";
        }

        const li = document.createElement("li");
        if (tag !== "") {
            const tagElement = document.createElement("span");
            tagElement.setAttribute("class", "tag");
            tagElement.appendChild(document.createTextNode(tag));
            li.appendChild(tagElement);
        }
        li.appendChild(link);
        ul.appendChild(li);
    }

    if (results.length === 0) {
        const li = document.createElement("li");
        li.appendChild(document.createTextNode("No result."));
        ul.appendChild(li);
    }

}

function search() {
    if (window.event.key !== 'Enter') {
        return;
    }

    if (g_toc === null) {
        fetch("/toc/" + g_language_code + ".json")
            .then(response => response.json())
            .then(json => {
                g_toc = json;
                doSearch();
            })
    } else {
        doSearch();
    }
}
