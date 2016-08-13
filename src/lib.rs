#![cfg_attr(test, deny(warnings))]
#![warn(trivial_casts)]
#![forbid(unused, unused_extern_crates, unused_import_braces, unused_qualifications)]

pub fn nav(subdomain: &str, page: &str, is_admin: bool) -> String {
    format!(
        r#"
<nav class="navbar navbar-inverse navbar-fixed-top" role="navigation">
    <div class="navbar-header">
        <button type="button" class="navbar-toggle" data-toggle="collapse" data-target=".navbar-ex1-collapse">
            <span class="sr-only">Toggle navigation</span>
            <span class="icon-bar"></span>
            <span class="icon-bar"></span>
            <span class="icon-bar"></span>
        </button>
        <a class="navbar-brand" href="{wiw}/">Willkommen in Wöllstein</a>
    </div>
    <div class="collapse navbar-collapse navbar-ex1-collapse">
        <ul id="navbar-list" class="nav navbar-nav">
            <li{active_home}><a href="{wiw}/"><span class="fa fa-home"></span>Home</a></li>
            <li{active_angebote}><a href="{wiw}/angebote"><span class="fa fa-smile-o"></span>Unsere Angebote</a></li>
            <li{active_veranstaltungen}><a href="{wiw}/veranstaltungen"><span class="fa fa-calendar"></span>Veranstaltungen</a></li>
            <li{active_presse}><a href="{wiw}/presse"><span class="fa fa-comments"></span>Presse</a></li>
            <li{active_galerie}><a href="{wiw}/galerie"><span class="fa fa-picture-o"></span>Galerie</a></li>
            <li{active_kontakt}><a href="{wiw}/kontakt"><span class="fa fa-envelope"></span>Kontakt</a></li>
            <li{active_boerse}><a href="{boerse}/"><span class="fa fa-exchange"></span>Börse</a></li>
        </ul>
        {right}
    </div>
</nav>
        "#,
        wiw=if subdomain == "boerse" { "//willkommeninwoellstein.de" } else { "" },
        boerse=if subdomain == "boerse" { "" } else { "//boerse.willkommeninwoellstein.de" },
        right=if subdomain == "boerse" && is_admin { r#"<ul class="navbar-boerse-admin nav navbar-nav navbar-right"><li><a href="/notiz/neu"><span class="fa fa-plus"></span>neue Notiz</a></li></ul>"# } else { "" },
        active_home=if subdomain != "boerse" && page == "/" { " class=\"active\"" } else { "" },
        active_angebote=if subdomain != "boerse" && page.starts_with("/angebote") { " class=\"active\"" } else { "" },
        active_veranstaltungen=if subdomain != "boerse" && page.starts_with("/veranstaltungen") { " class=\"active\"" } else { "" },
        active_presse=if subdomain != "boerse" && page.starts_with("/presse") { " class=\"active\"" } else { "" },
        active_galerie=if subdomain != "boerse" && page.starts_with("/galerie") { " class=\"active\"" } else { "" },
        active_kontakt=if subdomain != "boerse" && page.starts_with("/kontakt") { " class=\"active\"" } else { "" },
        active_boerse=if subdomain == "boerse" { " class=\"active\"" } else { "" },
    )
}