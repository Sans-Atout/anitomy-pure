//! File extension detection using a static set of known media and non-media extensions.
use std::{collections::HashSet, path::Path, sync::LazyLock};

static VALID_EXTENSIONS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        ".php",
        ".html",
        ".txt",
        ".htm",
        ".aspx",
        ".asp",
        ".js",
        ".css",
        ".pgsql.txt",
        ".mysql.txt",
        ".pdf",
        ".cgi",
        ".inc",
        ".gif",
        ".jpg",
        ".swf",
        ".xml",
        ".cfm",
        ".xhtml",
        ".wmv",
        ".zip",
        ".axd",
        ".gz",
        ".png",
        ".doc",
        ".shtml",
        ".jsp",
        ".ico",
        ".exe",
        ".csi",
        ".inc.php",
        ".config",
        ".jpeg",
        ".ashx",
        ".log",
        ".xls",
        ".old",
        ".mp3",
        ".com",
        ".tar",
        ".ini",
        ".asa",
        ".tgz",
        ".PDF",
        ".flv",
        ".php3",
        ".bak",
        ".rar",
        ".asmx",
        ".xlsx",
        ".page",
        ".phtml",
        ".dll",
        ".JPG",
        ".asax",
        ".msg",
        ".pl",
        ".GIF",
        ".ZIP",
        ".csv",
        ".css.aspx",
        ".JPEG",
        ".ppt",
        ".nsf",
        ".Pdf",
        ".Gif",
        ".bmp",
        ".sql",
        ".Jpeg",
        ".Jpg",
        ".xml.gz",
        ".Zip",
        ".new",
        ".avi",
        ".psd",
        ".rss",
        ".wav",
        ".action",
        ".db",
        ".dat",
        ".do",
        ".xsl",
        ".class",
        ".mdb",
        ".include",
        ".cs",
        ".class.php",
        ".htc",
        ".mov",
        ".tpl",
        ".js.php",
        ".mysql-connect",
        ".mpg",
        ".rdf",
        ".rtf",
        ".ascx",
        ".mvc",
        ".1.0",
        ".files",
        ".master",
        ".jar",
        ".vb",
        ".mp4",
        ".local.php",
        ".fla",
        ".require",
        ".de",
        ".docx",
        ".php5",
        ".wci",
        ".readme",
        ".cfg",
        ".aspx.cs",
        ".cfc",
        ".dwt",
        ".ru",
        ".LCK",
        ".Config",
        ".gif_var_DE",
        ".html_var_DE",
        ".net",
        ".ttf",
        ".HTM",
        ".X-AOM",
        ".jhtml",
        ".mpeg",
        ".ASP",
        ".LOG",
        ".X-FANCYCAT",
        ".php4",
        ".readme_var_DE",
        ".vcf",
        ".X-RMA",
        ".X-AFFILIATE",
        ".X-OFFERS",
        ".X-AFFILIATE_var_DE",
        ".X-AOM_var_DE",
        ".X-FANCYCAT_var_DE",
        ".X-FCOMP",
        ".X-FCOMP_var_DE",
        ".X-GIFTREG",
        ".X-GIFTREG_var_DE",
        ".X-MAGNIFIER",
        ".X-MAGNIFIER_var_DE",
        ".X-OFFERS_var_DE",
        ".X-PCONF",
        ".X-PCONF_var_DE",
        ".X-RMA_var_DE",
        ".X-SURVEY",
        ".tif",
        ".dir",
        ".json",
        ".6.9",
        ".Zif",
        ".wma",
        ".mid",
        ".rm",
        ".aspx.vb",
        ".tar.gz",
        ".woa",
        ".main",
        ".ram",
        ".opml",
        ".0.html",
        ".css.php",
        ".feed",
        ".lasso",
        ".6.3",
        ".shtm",
        ".sitemap",
        ".scc",
        ".tmp",
        ".backup",
        ".sln",
        ".org",
        ".conf",
        ".mysql-query",
        ".session-start",
        ".uk",
        ".TXT",
        ".orig",
        ".settings.php",
        ".cab",
        ".kml",
        ".lck",
        ".pps",
        ".require-once",
        ".asx",
        ".bok",
        ".msi",
        ".c",
        ".fcgi",
        ".fopen",
        ".html.",
        ".phpmailer.php",
        ".bin",
        ".htaccess",
        ".info",
        ".java",
        ".jsf",
        ".tmpl",
        ".DOC",
        ".bat",
        ".com.html",
        ".print",
        ".resx",
        ".ics",
        ".php.php",
        ".x",
        ".PNG",
        ".data",
        ".dcr",
        ".enfinity",
        ".html.html",
        ".licx",
        ".mno",
        ".plx",
        ".vm",
        ".5.php",
        ".HTML",
        ".MP3",
        ".config.php",
        ".dwg",
        ".edu",
        ".search",
        ".static",
        ".wws",
        ".6.edu",
        ".OLD",
        ".bz2",
        ".co.uk",
        ".ece",
        ".epc",
        ".getimagesize",
        ".ice",
        ".it_Backup_Giornaliero",
        ".it_Backup_Settimanale",
        ".jspa",
        ".lst",
        ".php-dist",
        ".svc",
        ".vbs",
        ".1.html",
        ".30-i486",
        ".ai",
        ".cur",
        ".dmg",
        ".img",
        ".inf",
        ".seam",
        ".smtp.php",
        ".1-bin-Linux-2.0.30-i486",
        ".7z",
        ".ajax",
        ".cfm.cfm",
        ".chm",
        ".csp",
        ".edit",
        ".file",
        ".gif.php",
        ".m3u",
        ".psp",
        ".py",
        ".sh",
        ".test",
        ".zdat",
        ".admin",
        ".captcha.aspx",
        ".dev",
        ".eps",
        ".file-get-contents",
        ".fr",
        ".fsockopen",
        ".list",
        ".m4v",
        ".min.js",
        ".new.html",
        ".p",
        ".store",
        ".webinfo",
        ".xml.php",
        ".BAK",
        ".htm.",
        ".php.bak",
        ".bk",
        ".bsp",
        ".cms",
        ".csshandler.ashx",
        ".d",
        ".html,",
        ".htmll",
        ".idx",
        ".images",
        ".jad",
        ".master.cs",
        ".prev_next",
        ".ssf",
        ".stm",
        ".txt.gz",
        ".Web.UI.WebResource.axd",
        ".as",
        ".asp.asp",
        ".au",
        ".cnf",
        ".dhtml",
        ".enu",
        ".html.old",
        ".include-once",
        ".lock",
        ".m",
        ".mysql-select-db",
        ".phps",
        ".pm",
        ".pptx",
        ".sav",
        ".sendtoafriendform",
        ".ssi",
        ".suo",
        ".vbproj",
        ".wml",
        ".xsd",
        ".26.13.391N35.50.38.816",
        ".26.24.165N35.50.24.134",
        ".26.56.247N35.52.03.605",
        ".27.02.940N35.49.56.075",
        ".27.15.919N35.52.04.300",
        ".27.29.262N35.47.15.083",
        ".3gp",
        ".40.00.573N35.42.57.445",
        ".43.58.040N35.38.35.826",
        ".44.04.344N35.38.35.077",
        ".44.08.714N35.39.08.499",
        ".44.10.892N35.38.49.246",
        ".44.27.243N35.41.29.367",
        ".44.29.976N35.37.51.790",
        ".44.32.445N35.36.10.206",
        ".44.34.800N35.38.08.156",
        ".44.37.128N35.40.54.403",
        ".44.40.556N35.40.53.025",
        ".44.45.013N35.38.36.211",
        ".44.46.104N35.38.22.970",
        ".44.48.130N35.38.25.969",
        ".44.52.162N35.38.50.456",
        ".44.58.315N35.38.53.455",
        ".45.01.562N35.38.38.778",
        ".45.04.359N35.38.39.112",
        ".45.06.789N35.38.22.556",
        ".45.10.717N35.38.41.989",
        ".ASPX",
        ".JS",
        ".PHP",
        ".array-keys",
        ".atom",
        ".award",
        ".bkp",
        ".crt",
        ".default",
        ".eml",
        ".epl",
        ".fancybox",
        ".fil",
        ".geo",
        ".h",
        ".hmtl",
        ".html.bak",
        ".ida",
        ".implode",
        ".index.php",
        ".iso",
        ".kmz",
        ".mysql-pconnect",
        ".php.old",
        ".php.txt",
        ".rec",
        ".storefront",
        ".taf",
        ".war",
        ".xslt",
        ".1.6",
        ".2a",
        ".8.1",
        ".CSS",
        ".NSF",
        ".Sponsors",
        ".a",
        ".aquery",
        ".ascx.cs",
        ".cat",
        ".contrib",
        ".ds",
        ".dwf",
        ".film",
        ".g",
        ".go",
        ".googlebook",
        ".gpx",
        ".hotelName",
        ".htm.htm",
        ".ihtml",
        ".in-array",
        ".index",
        ".ini.php",
        ".layer",
        ".maninfo",
        ".odt",
        ".price",
        ".randomhouse",
        ".read",
        ".ru-tov.html",
        ".s7",
        ".sample",
        ".sit",
        ".src",
        ".tpl.php",
        ".trck",
        ".uguide",
        ".vorteil",
        ".wbp",
        ".2.html",
        ".AVI",
        ".Asp",
        ".EXE",
        ".WMV",
        ".asax.vb",
        ".aspx.aspx",
        ".btr",
        ".cer",
        ".common.php",
        ".de.html",
        ".html\u{200e}",
        ".jbf",
        ".lbi",
        ".lib.php",
        ".lnk",
        ".login",
        ".login.php",
        ".mhtml",
        ".mpl",
        ".mso",
        ".mysql-result",
        ".original",
        ".pgp",
        ".ph",
        ".php.",
        ".preview",
        ".preview-content.php",
        ".search.htm",
        ".site",
        ".text",
        ".view",
        ".3.html",
        ".4.html",
        ".5.html",
        ".ICO",
        ".Web",
        ".XLS",
        ".action2",
        ".asc",
        ".asp.bak",
        ".aspx.resx",
        ".browse",
        ".code",
        ".com_Backup_Giornaliero",
        ".com_Backup_Settimanale",
        ".csproj",
        ".dtd",
        ".en.html",
        ".ep",
        ".eu",
        ".form",
        ".html1",
        ".inc.asp",
        ".index.html",
        ".it",
        ".nl",
        ".ogg",
        ".old.php",
        ".old2",
        ".opendir",
        ".out",
        ".pgt",
        ".php,",
        ".php\u{200e}",
        ".po",
        ".prt",
        ".query",
        ".rb",
        ".rhtml",
        ".ru.html",
        ".save",
        ".search.php",
        ".t",
        ".wsdl",
        ".0-to1.2.php",
        ".CFM",
        ".MOV",
        ".MPEG",
        ".Master",
        ".PPT",
        ".TTF",
        ".Templates",
        ".XML",
        ".adp",
        ".ajax.php",
        ".apsx",
        ".asf",
        ".bck",
        ".bu",
        ".calendar",
        ".captcha",
        ".cart",
        ".com.crt",
        ".core",
        ".dict.php",
        ".dot",
        ".egov",
        ".en.php",
        ".eot",
        ".errors",
        ".f4v",
        ".fr.html",
        ".git",
        ".ht",
        ".hta",
        ".html.LCK",
        ".html.printable",
        ".ini.sample",
        ".lib",
        ".lic",
        ".map",
        ".master.vb",
        ".mi",
        ".mkdir",
        ".o",
        ".p7b",
        ".pac",
        ".parse.errors",
        ".pd",
        ".pfx",
        ".php2",
        ".php_files",
        ".phtm",
        ".png.php",
        ".portal",
        ".printable",
        ".psql",
        ".pub",
        ".q",
        ".ra",
        ".reg",
        ".restrictor.php",
        ".rpm",
        ".strpos",
        ".tcl",
        ".template",
        ".tiff",
        ".tv",
        ".us",
        ".user",
        ".Controls",
        ".WAV",
        ".acgi",
        ".alt",
        ".array-merge",
        ".back",
        ".call-user-func-array",
        ".cfml",
        ".cmd",
        ".cocomore.txt",
        ".detail",
        ".disabled",
        ".dist.php",
        ".djvu",
        ".dta",
        ".e",
        ".extract",
        ".file-put-contents",
        ".fpl",
        ".framework",
        ".fread",
        ".htm.LCK",
        ".inc.js",
        ".includes",
        ".jp",
        ".jpg.html",
        ".l",
        ".letter",
        ".local",
        ".num",
        ".pem",
        ".php.sample",
        ".php}",
        ".php~",
        ".pot",
        ".preg-match",
        ".process",
        ".ps",
        ".r",
        ".raw",
        ".rc",
        ".s",
        ".search.",
        ".server",
        ".sis",
        ".sql.gz",
        ".squery",
        ".subscribe",
        ".svg",
        ".svn",
        ".thtml",
        ".tpl.html",
        ".ua",
        ".vcs",
        ".xhtm",
        ".xml.asp",
        ".xpi",
        ".A",
        ".PAGE",
        ".SWF",
        ".add",
        ".array-rand",
        ".asax.cs",
        ".asax.resx",
        ".ascx.vb",
        ".aspx,",
        ".aspx.",
        ".awm",
        ".b",
        ".bhtml",
        ".bml",
        ".ca",
        ".cache",
        ".cfg.php",
        ".cn",
        ".cz",
        ".de.txt",
        ".diff",
        ".email",
        ".en",
        ".error",
        ".faces",
        ".filesize",
        ".functions.php",
        ".hml",
        ".hqx",
        ".html,404",
        ".html.php",
        ".htmls",
        ".htx",
        ".i",
        ".idq",
        ".jpe",
        ".js.aspx",
        ".js.gz",
        ".jspf",
        ".load",
        ".media",
        ".mp2",
        ".mspx",
        ".mv",
        ".mysql",
        ".new.php",
        ".ocx",
        ".oui",
        ".outcontrol",
        ".pad",
        ".pages",
        ".pdb",
        ".pdf.",
        ".pnp",
        ".pop_formata_viewer",
        ".popup.php",
        ".popup.pop_formata_viewer",
        ".pvk",
        ".restrictor.log",
        ".results",
        ".run",
        ".scripts",
        ".sdb",
        ".ser",
        ".shop",
        ".sitemap.xml",
        ".smi",
        ".start",
        ".ste",
        ".swf.swf",
        ".templates",
        ".textsearch",
        ".torrent",
        ".unsubscribe",
        ".v",
        ".vbproj.webinfo",
        ".web",
        ".wmf",
        ".wpd",
        ".ws",
        ".xpml",
        ".y",
        ".AdCode",
        ".Aspx",
        ".C.",
        ".COM",
        ".GetMapImage",
        ".Html",
        ".Run.AdCode",
        ".Skins",
        ".Z",
        ".access.login",
        ".ajax.asp",
        ".app",
        ".asd",
        ".asm",
        ".assets",
        ".at",
        ".bad",
        ".bak2",
        ".blog",
        ".casino",
        ".cc",
        ".cdr",
        ".changeLang.php",
        ".children",
        ".com,",
        ".com-redirect",
        ".content",
        ".copy",
        ".count",
        ".cp",
        ".csproj.user",
        ".custom",
        ".dbf",
        ".deb",
        ".delete",
        ".details.php",
        ".dic",
        ".divx",
        ".download",
        ".download.php",
        ".downloadCirRequirements.pdf",
        ".downloadTourkitRequirements.pdf",
        ".emailCirRequirements.php",
        ".emailTourkitForm.php",
        ".emailTourkitNotification.php",
        ".emailTourkitRequirements.php",
        ".epub",
        ".err",
        ".es",
        ".exclude",
        ".filemtime",
        ".fillPurposes2.php",
        ".grp",
        ".home",
        ".htlm",
        ".htm,",
        ".html-",
        ".image",
        ".inc.html",
        ".it.html",
        ".j",
        ".jnlp",
        ".js.asp",
        ".js2",
        ".jspx",
        ".lang-en.php",
        ".link",
        ".listevents",
        ".log.0",
        ".mbox",
        ".mc_id",
        ".menu.php",
        ".mgi",
        ".mod",
        ".net.html",
        ".news",
        ".none",
        ".off",
        ".p3p",
        ".php.htm",
        ".php.static",
        ".php1",
        ".phpp",
        ".pop3.php",
        ".pop_3D_viewer",
        ".popup.pop_3D_viewer",
        ".prep",
        ".prg",
        ".print.html",
        ".print.php",
        ".product_details",
        ".pwd",
        ".pyc",
        ".red",
        ".registration",
        ".requirementsFeesTable.php",
        ".roshani-gunewardene.com",
        ".se",
        ".sea",
        ".sema",
        ".session",
        ".setup",
        ".simplexml-load-file",
        ".sitx",
        ".smil",
        ".srv",
        ".swi",
        ".swp",
        ".sxw",
        ".tar.bz2",
        ".tem",
        ".temp",
        ".template.php",
        ".top",
        ".txt.php",
        ".types",
        ".unlink",
        ".url",
        ".userLoginPopup.php",
        ".visaPopup.php",
        ".visaPopupValid.php",
        ".vspscc",
        ".vssscc",
        ".w",
        ".work",
        ".wvx",
        ".xspf",
        ".-",
        ".Admin",
        ".E.",
        ".Engineer",
        ".INC",
        ".LOG.new",
        ".MAXIMIZE",
        ".MPG",
        ".NDM",
        ".Php",
        ".R",
        ".SIM",
        ".SQL",
        ".Services",
        ".[file",
        ".accdb",
        ".act",
        ".actions.php",
        ".admin.php",
        ".ads",
        ".alhtm",
        ".all",
        ".ani",
        ".apf",
        ".apj",
        ".ar",
        ".aral-design.com",
        ".aral-design.de",
        ".arc",
        ".array-key-exists",
        ".asp.old",
        ".asp1",
        ".aspg",
        ".bfhtm",
        ".biminifinder",
        ".br",
        ".browser",
        ".build",
        ".buscar",
        ".categorias",
        ".categories",
        ".ccs",
        ".ch",
        ".cl",
        ".click.php",
        ".cls",
        ".cls.php",
        ".cms.ad.AdServer.cls",
        ".com-tov.html",
        ".com.ar",
        ".com.br",
        ".com.htm",
        ".com.old",
        ".common",
        ".conf.php",
        ".contact.php",
        ".control",
        ".core.php",
        ".counter.php",
        ".coverfinder",
        ".create.php",
        ".cs2",
        ".d2w",
        ".dbm",
        ".dct",
        ".dmb",
        ".doc.doc",
        ".dxf",
        ".ed",
        ".email.shtml",
        ".en.htm",
        ".engine",
        ".env",
        ".error-log",
        ".esp",
        ".ex",
        ".exc",
        ".exe,",
        ".ext",
        ".external",
        ".ficheros",
        ".fichiers",
        ".flush",
        ".fmt",
        ".fn",
        ".footer",
        ".form_jhtml",
        ".friend",
        ".g.",
        ".geo.xml",
        ".ghtml",
        ".google.com",
        ".gov",
        ".gpg",
        ".hl",
        ".href",
        ".htm.d",
        ".htm.html",
        ".htm.old",
        ".htm2",
        ".html.orig",
        ".html.sav",
        ".html[",
        ".html]",
        ".html_",
        ".html_files",
        ".htmlpar",
        ".htmlprint",
        ".html}",
        ".htm~",
        ".hts",
        ".hu",
        ".hwp",
        ".ibf",
        ".il",
        ".image.php",
        ".imagecreatetruecolor",
        ".imagejpeg",
        ".iml",
        ".imprimer",
        ".imprimer-cadre",
        ".imprimir",
        ".imprimir-marco",
        ".info.html",
        ".info.php",
        ".ini.bak",
        ".ini.default",
        ".inl",
        ".inv",
        ".join",
        ".jpg.jpg",
        ".jps",
        ".key",
        ".kit",
        ".lang",
        ".lignee",
        ".ltr",
        ".lzh",
        ".m4a",
        ".mail",
        ".manager",
        ".md5",
        ".met",
        ".metadesc",
        ".metakeys",
        ".mht",
        ".min",
        ".mld",
        ".mobi",
        ".mobile",
        ".mv4",
        ".n",
        ".net-tov.html",
        ".nfo",
        ".nikon",
        ".nodos",
        ".nxg",
        ".obyx",
        ".ods",
        ".old.2",
        ".old.asp",
        ".old.html",
        ".open",
        ".opml.config",
        ".ord",
        ".org.zip",
        ".ori",
        ".partfinder",
        ".pho",
        ".php-",
        ".phpl",
        ".phpx",
        ".pix",
        ".pls",
        ".prc",
        ".pre",
        ".prhtm",
        ".print-frame",
        ".print.",
        ".print.shtml",
        ".printer",
        ".properties",
        ".propfinder",
        ".pvx",
        ".recherche",
        ".redirect",
        ".req",
        ".roshani-gunewardene.net",
        ".roshani-m-gunewardene.com",
        ".safe",
        ".sbk",
        ".se.php",
        ".search.asp",
        ".sec",
        ".seo",
        ".serv",
        ".server.php",
        ".servlet",
        ".settings",
        ".sf",
        ".shopping_return.php",
        ".shopping_return_adsense.php",
        ".show",
        ".sht",
        ".skins",
        ".so",
        ".sph",
        ".split",
        ".sso",
        ".stats.php",
        ".story",
        ".swd",
        ".swf.html",
        ".sys",
        ".tex",
        ".tga",
        ".thm",
        ".tlp",
        ".tml",
        ".tmp.php",
        ".touch",
        ".tsv",
        ".txt.",
        ".txt.html",
        ".ug",
        ".unternehmen",
        ".utf8",
        ".vbproj.vspscc",
        ".vsprintf",
        ".vstemplate",
        ".vtl",
        ".wbmp",
        ".webc",
        ".webproj",
        ".wihtm",
        ".wp",
        ".wps",
        ".wri",
        ".wsc",
        ".www",
        ".xsp",
        ".xsql",
        ".zip,",
        ".zml",
        ".ztml",
        ". EXTRAHOTELERO HOSPEDAJE",
        ". T.",
        ". php",
        ".,",
        ".a5w",
        ".aac",
        ".access",
        ".act.php",
        ".action.php",
        ".actions",
        ".activate.php",
        ".ad.php",
        ".add.php",
        ".adenaw.com",
        ".adm",
        ".advsearch",
        ".ag.php",
        ".aj_",
        ".all.hawaii",
        ".amaphun.com",
        ".andriy.lviv.ua",
        ".ap",
        ".api",
        ".apk",
        ".application",
        ".archiv",
        ".arj",
        ".array-map",
        ".array-values",
        ".art",
        ".artdeco",
        ".articlePk",
        ".artnet.",
        ".ascx.resx",
        ".asia",
        ".asp-",
        ".asp.LCK",
        ".asp.html",
        ".asp2",
        ".aspDONOTUSE",
        ".asp_",
        ".asp_files",
        ".aspl",
        ".aspp",
        ".asps",
        ".aspx.designer.cs",
        ".aspx_files",
        ".aspxx",
        ".aspy",
        ".asxp",
        ".at.html",
        ".avatar.php",
        ".awstats",
        ".babymhiasexy.com",
        ".backup.php",
        ".bak.php",
        ".banan.se",
        ".banner.php",
        ".barnes",
        ".basicmap.php",
        ".baut",
        ".bc",
        ".best-vpn.com",
        ".beta",
        ".biz",
        ".blackandmature.com",
        ".bmp.php",
        ".board.asd",
        ".boom",
        ".bossspy.org",
        ".buscadorpornoxxx.com",
        ".buy-here.com",
        ".buyadspace",
        ".bycategory",
        ".bylocation",
        ".bz",
        ".c.html",
        ".cache.inc.php",
        ".cache.php",
        ".car",
        ".cascinaamalia.it",
        ".cat.php",
        ".catalog",
        ".cdf",
        ".ce",
        ".cfm.bak",
        ".cfsifatest.co.uk",
        ".cfstest.co.uk",
        ".cfswf",
        ".cfx",
        ".cgis",
        ".chat",
        ".chdir",
        ".chloesworld.com",
        ".classes.php",
        ".cmp",
        ".cnt",
        ".co",
        ".co-operativebank.co.uk",
        ".co-operativebanktest.co.uk",
        ".co-operativeinsurance.co.uk",
        ".co-operativeinsurancetest.co.uk",
        ".co-operativeinvestmentstest.co.uk",
        ".co.il",
        ".colorbox-min.js",
        ".com-authorization-required.html",
        ".com-bad-request.html",
        ".com-forbidden.html",
        ".com-internal-server-error.html",
        ".com-page-not-found.html",
        ".com.au",
        ".com.php",
        ".com.ua",
        ".com_Backup_",
        ".com_files",
        ".comments",
        ".comments.",
        ".comments.php",
        ".compiler.php",
        ".conf.html",
        ".confirm.email",
        ".connect.php",
        ".console",
        ".contact",
        ".content.php",
        ".controller",
        ".controls-3.1.5.swf",
        ".cookie.js",
        ".corp",
        ".corp.footer",
        ".cqs",
        ".cron",
        ".cropcanvas.php",
        ".cropinterface.php",
        ".crx",
        ".csproj.webinfo",
        ".csr",
        ".css.LCK",
        ".css.gz",
        ".cssd",
        ".csv.php",
        ".ctp",
        ".cx",
        ".cycle.all.min.js",
        ".d64",
        ".daisy",
        ".dal",
        ".daniel",
        ".daniel-sebald.de",
        ".data.php",
        ".data_",
        ".davis",
        ".dbml",
        ".dcf",
        ".de.jsp",
        ".default.php",
        ".del",
        ".deleted",
        ".dell",
        ".demo",
        ".desarrollo.aquihaydominios.com",
        ".dev.bka.co.nz",
        ".development",
        ".dig",
        ".display.php",
        ".dist",
        ".dk",
        ".dm",
        ".dmca-sucks.com",
        ".dms",
        ".dnn",
        ".dogpl",
        ".donothiredandobrin.com",
        ".dontcopy",
        ".downloadfreeporn.asia",
        ".du",
        ".dump",
        ".dws",
        ".dyn",
        ".ea3ny.com",
        ".easing.min.js",
        ".ebay",
        ".ebay.results.html",
        ".editingoffice.com",
        ".efacil.com.br",
        ".ehtml",
        ".emaximinternational.com",
        ".en.jsp",
        ".enn",
        ".equonix.com",
        ".es.html",
        ".es.jsp",
        ".euforyou.net",
        ".eur",
        ".excel.xml.php",
        ".exec",
        ".exp",
        ".f.l.",
        ".faucetdepot",
        ".faucetdepot.com.vbproj",
        ".faucetdepot.com.vbproj.webinfo",
        ".fb2",
        ".fdml",
        ".feeds.php",
        ".ffa",
        ".ficken.cx",
        ".filereader",
        ".filters.php",
        ".flac",
        ".flypage",
        ".fon",
        ".forget.pass",
        ".form.php",
        ".forms",
        ".forum",
        ".found",
        ".fp7",
        ".fr.jsp",
        ".freeasianporn.asia",
        ".freepornxxx.asia",
        ".frk",
        ".frontpage.php",
        ".ft",
        ".ftl",
        ".fucks.nl",
        ".funzz.fr",
        ".gallery.php",
        ".garcia",
        ".gb",
        ".get",
        ".get-meta-tags",
        ".gif         ",
        ".gif.count",
        ".girlvandiesuburbs.co.za",
        ".gitihost.com",
        ".glasner.ru",
        ".google",
        ".gray",
        ".gsp",
        ".guiaweb.tk",
        ".gutschein",
        ".guy",
        ".ha",
        ".hardestlist.com",
        ".hardpussy.com",
        ".hasrett.de",
        ".hawaii",
        ".header.php",
        ".henry",
        ".him",
        ".history",
        ".hlr",
        ".hm",
        ".ho",
        ".hokkaido",
        ".hold",
        ".home.php",
        ".home.test",
        ".homepage",
        ".hp",
        ".htm.bak",
        ".htm.rc",
        ".htm3",
        ".htm5",
        ".htm7",
        ".htm8",
        ".htm_",
        ".html,,",
        ".html-0",
        ".html-1",
        ".html-c",
        ".html-old",
        ".html-p",
        ".html.htm",
        ".html.images",
        ".html.inc",
        ".html.none",
        ".html.pdf",
        ".html.start",
        ".html.txt",
        ".html4",
        ".html5",
        ".html7",
        ".htmlBAK",
        ".htmlDolmetschen",
        ".html_old",
        ".htmla",
        ".htmlc",
        ".htmlfeed",
        ".htmlq",
        ".htmlu",
        ".htn",
        ".htpasswd",
        ".iac.",
        ".ibuysss.info",
        ".iconv",
        ".idf",
        ".iframe_filtros",
        ".ignore.php",
        ".ihmtl",
        ".ihya",
        ".imp",
        ".in",
        ".inactive",
        ".inc.php.bak",
        ".inc.php3",
        ".incest-porn.sex-startje.nl",
        ".incestporn.sex-startje.nl",
        ".incl",
        ".indiansexzite.com",
        ".indt",
        ".ini.NEWCONFIGPOSSIBLYBROKEN",
        ".insert",
        ".internet-taxprep.com",
        ".interpreterukraine.com",
        ".ipl",
        ".issues",
        ".itml",
        ".ixi",
        ".jhtm",
        ".job",
        ".joseph",
        ".jpf",
        ".jpg.xml",
        ".jpg[",
        ".jpg]",
        ".js,",
        ".js.LCK",
        ".jsa",
        ".jsd",
        ".jso",
        ".jsp.old",
        ".jsps",
        ".jtp",
        ".keyword",
        ".kinkywear.net",
        ".kk",
        ".knvbcommunicator.voetbalassist.nl",
        ".kokuken",
        ".ks",
        ".kutxa.net-en",
        ".lang-de.php",
        ".lang.php",
        ".langhampartners.com",
        ".lappgroup.com",
        ".last",
        ".latest",
        ".lha",
        ".links",
        ".list.includes",
        ".listMiniGrid",
        ".listing",
        ".lng",
        ".loc",
        ".local.cfm",
        ".location.href",
        ".log2",
        ".lua",
        ".lynkx",
        ".maastrichtairporthotels.com",
        ".mag",
        ".mail.php",
        ".malesextoys.us",
        ".massivewankers.com",
        ".mbizgroup",
        ".mel",
        ".members",
        ".meretrizdelujo.com",
        ".messagey.com",
        ".metadata.js",
        ".meus.php",
        ".midi",
        ".milliculture.net",
        ".min_",
        ".miss-video.com",
        ".mk.gutschein",
        ".mk.rabattlp",
        ".mkv",
        ".mmap",
        ".model-escorts.asia",
        ".modelescorts.asia",
        ".mp",
        ".mp3.html",
        ".mq4",
        ".mreply.rc",
        ".msp",
        ".mvn",
        ".mysqli",
        ".napravlenie_ASC",
        ".napravlenie_DESC",
        ".nded-pga-emial",
        ".net-en",
        ".net-print.htm",
        ".net_Backup_Giornaliero",
        ".net_Backup_Settimanale",
        ".new.htm",
        ".newsletter",
        ".nexucom.com",
        ".ninwinter.net",
        ".nl.html",
        ".nonude.org",
        ".nonudes.com",
        ".nth",
        ".nz",
        ".od",
        ".offer.php",
        ".offline",
        ".ogv",
        ".ok",
        ".old.1",
        ".old.htm",
        ".old.old",
        ".old1",
        ".old3",
        ".older",
        ".oliver",
        ".onedigitalcentral.com",
        ".onenettv.com",
        ".online",
        ".opensearch",
        ".org-tov.html",
        ".org.ua-tov.html",
        ".orig.html",
        ".origin.php",
        ".original.html",
        ".orlando-vacationhome.net",
        ".orlando-vacationhomes-pools.com",
        ".orlando-vacationrentals.net",
        ".osg",
        ".outbound",
        ".owen",
        ".ownhometest.co.uk",
        ".pae",
        ".page_pls_all_password",
        ".pages-medicales.com",
        ".pan",
        ".parse-url",
        ".part",
        ".pass",
        ".patch",
        ".paul",
        ".paymethods.php",
        ".pazderski.com",
        ".pazderski.net",
        ".pazderski.us",
        ".pdd",
        ".pdf.html",
        ".pdf.pdf",
        ".pdf.php",
        ".pdfx",
        ".perfect-color-world.com",
        ".petersburg-apartments-for-business.html",
        ".petersburg-apartments-for-tourists.html",
        ".petersburg-romantic-apartments.html",
        ".phdo",
        ".photo",
        ".php--------------",
        ".php.LCK",
        ".php.backup",
        ".php.html",
        ".php.inc",
        ".php.mno",
        ".php.original",
        ".php_",
        ".php_OLD",
        ".php_old",
        ".phphp",
        ".phppar",
        ".phpvreor.php",
        ".php",
        ".pht",
        ".pl.html",
        ".planetcom.ca",
        ".playwithparis.com",
        ".plugins",
        ".png,bmp",
        ".popup",
        ".pornfailures.com",
        ".pornoizlee.tk",
        ".pornz.tv",
        ".posting.prep",
        ".prev",
        ".print.jsp",
        ".prl",
        ".prosdo.com",
        ".psb",
        ".publisher.php",
        ".puresolo.com",
        ".pussyjourney.com",
        ".qtgp",
        ".qxd",
        ".r.",
        ".rabattlp",
        ".rails",
        ".randomocityproductions.com",
        ".rateart.php",
        ".readfile",
        ".rec.html",
        ".redirect.php",
        ".remove",
        ".remove.php",
        ".removed",
        ".resultados",
        ".resume",
        ".rhtm",
        ".riddlesintime.com",
        ".rmvb",
        ".ro",
        ".roma",
        ".roomscity.com",
        ".roshanigunewardene.com",
        ".rpt",
        ".rsp",
        ".rss.php",
        ".rss_cars",
        ".rss_homes",
        ".rss_jobs",
        ".rtfd",
        ".rvt",
        ".s.html",
        ".sadopasion.com",
        ".safariextz",
        ".salestax.php",
        ".sc",
        ".sca-tork.com",
        ".scandir",
        ".scrollTo.js",
        ".search.html",
        ".sec.cfm",
        ".section",
        ".secure",
        ".send",
        ".sent-",
        ".service",
        ".session-regenerate-id",
        ".set",
        ".sex-startje.nl",
        ".sexmeme.com",
        ".sexon.com",
        ".sexy-girls4abo.de",
        ".sfw",
        ".sgf",
        ".shipcode.php",
        ".shipdiscount.php",
        ".show.php",
        ".shtml.html",
        ".sidebar",
        ".sisx",
        ".sitemap.",
        ".skin",
        ".small-penis-humiliation.net",
        ".smiletest.co.uk",
        ".snippet.aspx",
        ".snuffx.com",
        ".sort",
        ".sortirovka_Price.napravlenie_ASC",
        ".sortirovka_Price.napravlenie_DESC",
        ".sortirovka_customers_rating.napravlenie_ASC",
        ".sortirovka_customers_rating.napravlenie_DESC",
        ".sortirovka_name.napravlenie_ASC",
        ".sortirovka_name.napravlenie_DESC",
        ".sp",
        ".sphp3",
        ".srch",
        ".srf",
        ".srvl",
        ".st-patricks.com",
        ".sta",
        ".staged.php",
        ".staging",
        ".start.php",
        ".stat",
        ".stats",
        ".step",
        ".stml",
        ".storebanner.php",
        ".storelogo.php",
        ".storename.php",
        ".sts.php",
        ".suarez",
        ".submit",
        ".support",
        ".support.html",
        ".swf.LCK",
        ".sym",
        ".system",
        ".tab-",
        ".table.html",
        ".tablesorter.min.js",
        ".tablesorter.pager.js",
        ".tatianyc.com",
        ".tb",
        ".tech",
        ".teen-shy.com",
        ".teenhardpussy.com",
        ".temp.php",
        ".templates.php",
        ".temporarily.withdrawn.html",
        ".test.cgi",
        ".test.php",
        ".tf",
        ".tg",
        ".thanks",
        ".thehotfish.com",
        ".theme",
        ".thompson",
        ".thumb.jpg",
        ".ticket.submit",
        ".tim",
        ".tk",
        ".tls",
        ".to",
        ".touch.action",
        ".trace",
        ".tracker.ashx",
        ".trade",
        ".trishasex.viedos.com",
        ".ts",
        ".tst",
        ".tvpi",
        ".txt.txt",
        ".txuri-urdin.com",
        ".ufo",
        ".ugmart.ug",
        ".ui-1.5.2",
        ".unixteacher.org",
        ".unsharp.php",
        ".update",
        ".upgrade",
        ".v1.11.js",
        ".v2.php",
        ".vacationhomes-pools.com",
        ".var",
        ".venetian.com,prod2.venetian.com,reservations.venetian.com,",
        ".verify",
        ".video",
        ".videodeputas.com",
        ".videos-chaudes.com",
        ".viewpage__10",
        ".vmdk",
        ".vn",
        ".voetbalassist.nl",
        ".vs",
        ".vx",
        ".vxlpub",
        ".w3m",
        ".w3x",
        ".wax",
        ".web-teck.com",
        ".webalizer",
        ".webarchive",
        ".webjockey.nl",
        ".webm",
        ".weedooz.eu",
        ".wgx",
        ".wimzi.php",
        ".wireless",
        ".wireless.action",
        ".wm",
        ".woolovers.com",
        ".working",
        ".wpl",
        ".wplus",
        ".wps.rtf",
        ".write.php",
        ".wwsec_app_priv.login",
        ".www.annuaire-vimarty.net",
        ".www.annuaire-web.info",
        ".www.kit-graphik.com",
        ".www.photo-scope.fr",
        ".xcam.at",
        ".xconf",
        ".xcwc.com",
        ".xgi",
        ".xhtml5",
        ".xlt",
        ".xm",
        ".xml.old",
        ".xpdf",
        ".xqy",
        ".xslx",
        ".xst",
        ".xsx",
        ".xy.php",
        ".yp",
        ".ys",
        ".z",
        ".za",
        ".zh.html",
        ".zhtml",
        ".zip.php",
    ]
    .into_iter()
    .collect()
});

fn is_valid_extension(tested_extension: &str) -> bool {
    VALID_EXTENSIONS.contains(tested_extension)
}

/// Strips the file extension from `file_name`, returning the stem.
pub(crate) fn remove_extension(file_name: &str) -> String {
    match get_extension(file_name) {
        Some(ext) => file_name
            .strip_suffix(&format!(".{}", ext))
            .unwrap_or(file_name)
            .to_owned(),
        None => file_name.to_owned(),
    }
}

/// Returns the file extension of `file_name` if it is a known media format, otherwise `None`.
pub(crate) fn get_extension(file_name: &str) -> Option<String> {
    let path = Path::new(file_name);
    let p_extension = path.extension()?;
    let string_extension = format!(".{}", p_extension.to_str().unwrap());
    if !is_valid_extension(&string_extension) {
        return None;
    }
    Some(p_extension.to_str().unwrap().to_string())
}

#[test]
fn test_valid_extension() {
    assert!(is_valid_extension(".mkv"));
    assert!(is_valid_extension(".mp4"));
    assert!(is_valid_extension(".avi"));
    assert!(is_valid_extension(".html"));
}

#[test]
fn test_invalid_extension() {
    assert!(!is_valid_extension("mkv"));
    assert!(!is_valid_extension(".non an extension"));
    assert!(!is_valid_extension("avi"));
}

#[cfg(test)]
mod tests {
    use super::{get_extension, remove_extension};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_valid_extension() {
        assert!(super::is_valid_extension(".mkv"));
        assert!(super::is_valid_extension(".mp4"));
        assert!(super::is_valid_extension(".avi"));
        assert!(super::is_valid_extension(".html"));
    }

    #[test]
    fn get_extension_mkv() {
        assert_eq!(get_extension("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv").unwrap(),"mkv");
        assert_eq!(
            get_extension("[ANBU]_Princess_Lover!_-_01_[2048A39A].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[ANBU-Menclave]_Canaan_-_01_[1024x576_H.264_AAC][12F00E89].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[ANBU-umai]_Haiyoru!_Nyaru-Ani_[596DD8E6].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[BakaWolf-m.3.3.w] Special A 01 (H.264) [C83164B9].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[chibi-Doki] Seikon no Qwaser - 13v0 (Uncensored Director's Cut) [988DB090].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Chihiro]_Kono_Aozora_ni_Yakusoku_Wo_10_v2_[DVD][h264][C83D206B].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Coalgirls]_Toradora_ED2_(704x480_DVD_AAC)_[3B65D1E6].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
        get_extension(
            "[Conclave-Mendoi]_Mobile_Suit_Gundam_00_S2_-_01v2_[1280x720_H.264_AAC][4863FBE8].mkv"
        )
        .unwrap(),
        "mkv"
    );
        assert_eq!(
            get_extension("[Frostii]_Nodame_Cantabile_Finale_-_00_[73AD0735].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[HorribleSubs] Tower of Druaga - Sword of Uruk - 04 [480p].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
        get_extension(
            "[Juuni.Kokki]-(Les.12.Royaumes)-[Ep.24]-[x264+OGG]-[JAP+FR+Sub.FR]-[Chap]-[AzF].mkv"
        )
        .unwrap(),
        "mkv"
    );
        assert_eq!(
            get_extension("[kito].Nazca.episode.01.DVDRip.[x264.He-aac.{Jpn}+Sub{Fr}].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[Lambda-Delta]_Umineko_no_Naku_Koro_ni_-_11_[848x480_H.264_AAC][943106AD].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[SS]_Kemono_no_Souja_Erin_-_12_(1280x720_h264)_[0F5F884F].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[UTW-TMD]_Summer_Wars_[BD][h264-720p][TrueHD5.1][9F311DAB].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[ValdikSS]_First_Squad_The_Morment_Of_Truth_[720x576_h264_dvdscr_eng_hardsub].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "Code_Geass_R2_TV_[20_of_25]_[ru_jp]_[HDTV]_[Varies_&_Cuba77_&_AnimeReactor_RU].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "Evangelion_1.11_You_Are_(Not)_Alone_(2009)_[1080p,BluRay,x264,DTS-ES]_-_THORA.mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "Evangelion_1.11_You_Are_(Not)_Alone_[1080p,BluRay,x264,DTS-ES]_-_THORA.mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("Eve no Jikan 2 [88F4F7F0].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("Gin'iro_no_Kami_no_Agito_(2006)_[1080p,BluRay,x264,DTS]_-_THORA.mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("Magical Girl Lyrical Nanoha A's - 01.DVD[H264.AAC][DGz][7A8A7769].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(get_extension("Mobile_Suit_Gundam_00_Season_2_Ep07_A_Reunion_and_a_Parting_[1080p,BluRay,x264]_-_THORA.mkv").unwrap(),"mkv");
        assert_eq!(
            get_extension("Noein_[01_of_24]_[ru_jp]_[bodlerov_&_torrents_ru].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("ponyo_on_the_cliff_by_the_sea[h264.dts][niizk].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[Seto_Otaku]_AIKa_ZERO_OVA_-_01_[BD][1920x1080_H264-Flac][6730D40A].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[a4e]R.O.D_the_TV_01[divx5.2.1].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(get_extension("Ghost_in_the_Shell_Stand_Alone_Complex_2nd_GIG_Ep05v2_EXCAVATION_[720p,HDTV,x264,AAC_5.1]_-_THORA.mkv").unwrap(),"mkv");
        assert_eq!(get_extension("Ghost_in_the_Shell_Stand_Alone_Complex_2nd_GIG_Ep06_Pu239_[720p,HDTV,x264,AAC_5.1]_-_THORA.mkv").unwrap(),"mkv");
        assert_eq!(
            get_extension("Fate_Stay_Night_Ep05_The_Two_Magi_Part1_[720p,BluRay,x264]_-_THORA.mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[RaX]Mezzo(DSA)_-_05_-_[x264_ogg]_[585d9971].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[AKH-SWE]_Fullmetal_Alchemist_(2009)_02v2_[H.264.AAC][7B2C5E8B].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[FuktLogik][Sayonara_Zetsubou_Sensei][01][DVDRip][x264_AC3].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Ayu]_Kiddy_Grade_2_-_Pilot_[H264_AC3][650B731B].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[Darksoul-Subs] Tatakau Shisho - The Book of Bantorra [848x480 XVID_MP3].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(get_extension("[ACX]Neon_Genesis_Evangelion_-_Platinum_-_06_-_Showdown_in_Tokyo_3_[SaintDeath]_[CBDB8577].mkv").unwrap(),"mkv");
        assert_eq!(
            get_extension("[Himatsubushi]_Sora_no_Woto_-_01_-_H264_-_720p_-_E83AD672.mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[EroGaKi-Team]_Nurse_Witch_Komugi-chan_Magikarte_02.5_[902BB314].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("Ookiku Furikabutte S2 - 09 (Central Anime) [BD841253].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[HorribleSubs] HEROMAN - 10_(XviD_AnimeSenshi).mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("Detective Conan - 316-317 [DCTP][2411959B].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[N LogN Fansubs] Angel Beats (9).mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("Juuousei_-_01_[Black_Sheep][HDTV_H264_AAC][803DA487].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[gg]_Kimi_ni_Todoke_2nd_Season_-_00_[BF735BC4].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("K-ON!_Ep03_Training!_[1080p,BluRay,x264]_-_THORA.mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("K-ON!!_Ep08_Career_Plan!_[1080p,BluRay,x264]_-_THORA.mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[E-HARO Raws] Kore wa Zombie desu ka - 03 (TV 1280x720 h264 AAC) [888E4991].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Edomae Subs] Kore wa Zombie desu ka  Episode 2.mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("5_centimeters_per_second[1904x1072.h264.flac][niizk].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Yoroshiku]_009-1_-_02_(H264)_[36D2444D].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("After War Gundam X - 1x03 - My Mount is Fierce!.mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[HorribleSubs] The World God Only Knows 2 - 03 [720p].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "Macross Frontier - Sayonara no Tsubasa (Central Anime, 720p) [46B35E25].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[FFF] Red Data Girl - 10v0 [29EA865B].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[CMS] Magical☆Star Kanon 100% OVA[DVD][E9F43685].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Doremi].Ro-Kyu-Bu!.SS.01.[C1B5CE5D].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(get_extension("[Raizel] Persona 4 The Animation Episode 13 - A Stormy Summer Vacation Part 1  [BD_1080p_Dual_Audio_FLAC_Hi10p][8A45634B].mkv").unwrap(),"mkv");
        assert_eq!(get_extension("[Hien] Kotoura-san - Special Short Anime 'Haruka's Room' - 01 [BD 1080p H.264 10-bit AAC][6B6BE015].mkv").unwrap(),"mkv");
        assert_eq!(
            get_extension("[R-R] Diebuster.EP1 (720p.Hi10p.AC3) [82E36A36].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("Aim_For_The_Top!_Gunbuster-ep1.BD(H264.FLAC.10bit)[KAA][69ECCDCF].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Rakuda].Gift.~eternal.rainbow~.01.dvd.h.264.vorbis.mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[Jumonji-Giri]_[Shinsen-Subs][ASF]_D.C.II_Da_Capo_II_Ep01_(a1fc58a7).mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[52wy][SlamDunk][001][Jpn_Chs_Cht][x264_aac][DVDRip][7FE2C873].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Commie] Last Exile ~Fam, The Silver Wing~ - 13 [AFF9E530].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[UTW]_Fate_Zero_-_01_[BD][h264-720p_AC3][02A0491D].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[UTW-THORA] Evangelion 3.33 You Can (Not) Redo [BD][1080p,x264,flac][F2060CF5].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "Evangelion Shin Gekijouban Q (BDrip 1920x1080 x264 FLACx2 5.1ch)-ank.mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "Howl's_Moving_Castle_(2004)_[1080p,BluRay,flac,dts,x264]_-_THORA v2.mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[FFF] Futsuu no Joshikousei ga [Locodol] Yatte Mita. - 01 [BAD09C76].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Asenshi] Rozen Maiden 3 - PV [CA57F300].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("Mary Bell (DVD) - 01v2 [h-b].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("Mary Bell (DVD) - 02 [h-b].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("The Irregular at Magic High School - S01E01- Enrollment Part I.mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Mezashite] Aikatsu! ‒ 100 [D035A39F].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[UTW]_Accel_World_-_EX01_[BD][h264-720p_AAC][3E56EE18].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[HorribleSubs] Tsukimonogatari - (01-04) [1080p].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Urusai]_Bokura_Ga_Ita_01_[DVD_h264_AC3]_[BFCE1627][Fixed].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Coalgirls]_Bakemonogatari_OP4a_(1280x720_Blu-Ray_FLAC)_[327A2375].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Ruri]No.6 01 [720p][H264][A956075C].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("EvoBot.[Watakushi]_Akuma_no_Riddle_-_01v2_[720p][69A307A2].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("01 - Land of Visible Pain.mkv").unwrap(),
            "mkv"
        );
        assert_eq!(get_extension("[Zom-B] Machine-Doll wa Kizutsukanai - 01 - Facing ''Cannibal Candy'' I (kuroi, FFF remux) [B99C8DED].mkv").unwrap(),"mkv");
        assert_eq!(
            get_extension("The iDOLM@STER 765 Pro to Iu Monogatari.mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Coalgirls]_Fate_Zero_OVA3.5_(1280x720_Blu-ray_FLAC)_[5F5AD026].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[GrimRipper] Koi Kaze [Dual Audio] Ep01 (c13cefe0).mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[HorribleSubs] Gintama - 111C [1080p].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[SpoonSubs]_Hidamari_Sketch_x365_-_04.1_(DVD)[B6CE8458].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Hatsuyuki]_Kuroko_no_Basuke_S3_-_01_(51)_[720p][10bit][619C57A0].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Elysium]Sora.no.Woto.EP07.5(BD.720p.AAC)[C37580F8].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(get_extension("[Zurako] Sora no Woto - 07.5 - Drinking Party - Fortress Battle (BD 1080p AAC) [F7DF16F7].mkv").unwrap(),"mkv");
        assert_eq!(
            get_extension("[Hiryuu] Maji de Watashi ni Koi Shinasai!! - 02 [720].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[Kira-Fansub] Uchuu no Stellvia ep 14 (BD H264 1280x960 24fps AAC) [06EE7355].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[ANE] Yosuga no Sora - Ep01 Preview (Yorihime ver) [BDRip 1080p x264 FLAC].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[B-G_&_m.3.3.w]_Myself_Yourself_12.DVD(H.264_DD2.0)_[CB2B37F1].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("The.Animatrix.08.A.Detective.Story.720p.BluRay.DTS.x264-ESiR.mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(get_extension("[DmonHiro] Oreshura #01v2 - The Start Of High School Life Is A War Zone [BD, 720p] [211375E6].mkv").unwrap(),"mkv");
        assert_eq!(get_extension("[NinjaPanda] Tiger & Bunny #01 All's well that ends well. (v3, 1080p Hi10P, DA AAC) [4A9AB85F].mkv").unwrap(),"mkv");
        assert_eq!(
            get_extension(
                "Neko no Ongaeshi - [HQR.remux-DualAudio][NTV.1280x692.h264](0CDC2145).mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[ReDone] Memories Off 3.5 - 04 (DVD 10-bit).mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[FFF] Seirei Tsukai no Blade Dance - SP01 [BD][720p-AAC][F1FF8588].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[Hatsuyuki] Dragon Ball Kai (2014) - 002 (100) [1280x720][DD66AFB7].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Deep] Tegami Bachi (REVERSE) - Letter Bee - 29 (04) [5203142B].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[FFF] Love Live! The School Idol Movie - PV [D1A15D2C].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[Nishi-Taku] Tamayura ~graduation photo~ Movie Part 1 [BD][720p][98965607].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(get_extension("[TardRaws] 0 [640x360].mkv").unwrap(), "mkv");
        assert_eq!(
            get_extension("[Hatsuyuki-Kaitou]_Fairy_Tail_2_-_52_(227)_[720p][10bit][9DF6B8D5].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[FBI] Baby Princess 3D Paradise Love 01v0 [BD][720p-AAC][457CC066].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[NamaeNai] Hidamari Sketch x365 - 09a (DVD) [49874745].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[FaggotryRaws] Bakuman - 01 (NHK E 848x480).mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("Dragon.Ball.KAI.-.01.-.1080p.BluRay.x264.DHD.mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[AnimeRG] Ushio to Tora (TV) - 02 [720p] [Xcelent].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[(´• ω •`)] Nintama Rantarou - S23E1821 - Buddhist Priest-sama is a Ninja.mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Judas] Aharen-san wa Hakarenai - S01E06v2.mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[0x539] Somali and the Forest Spirit - S01E01 (WEB 1080p Hi10P AAC) [BB7C6531].mkv"
            )
            .unwrap(),
            "mkv"
        );
    }
    #[test]
    fn get_extension_avi() {
        assert_eq!(
            get_extension("[DB]_Bleach_225_[C63D149C].avi").unwrap(),
            "avi"
        );
        assert_eq!(
            get_extension("[KAF-TEAM]_One_Piece_Movie_9_vostfr_HD.avi").unwrap(),
            "avi"
        );
        assert_eq!(
            get_extension("[RNA]_Sakura_Taisen_New_York_NY_Ep_2_[1590D378].avi").unwrap(),
            "avi"
        );
        assert_eq!(
            get_extension("[Ayako]_Infinite_Stratos_-_IS_-_01v2_[XVID][400p][29675B71].avi")
                .unwrap(),
            "avi"
        );
        assert_eq!(get_extension("Juuni.Kokki.Ep.5.avi").unwrap(), "avi");
        assert_eq!(get_extension("Juuni Kokki Ep.5.avi").unwrap(), "avi");
        assert_eq!(
            get_extension("[Keroro].148.[Xvid.mp3].[FE68D5F1].avi").unwrap(),
            "avi"
        );
        assert_eq!(
            get_extension("[Triad]_Today_in_Class_5-2_-_04.avi").unwrap(),
            "avi"
        );
        assert_eq!(get_extension("__BLUE DROP 10 (1).avi").unwrap(), "avi");
        assert_eq!(
            get_extension("37 [Ruberia]_Death_Note_-_37v2_[FINAL]_[XviD][6FA7D273].avi").unwrap(),
            "avi"
        );
        assert_eq!(get_extension("[FB] Crayon Shin-Chan Movie 2 The Secret of Buri Buri Kingdom [DivX5 AC3] 1994 [852X480] V2.avi").unwrap(),"avi");
        assert_eq!(
            get_extension("[Shinsen-Subs]_Macross_Frontier_-_01b_[4D5EC315].avi").unwrap(),
            "avi"
        );
        assert_eq!(get_extension("[KLF]_D.Gray-man_04V2.avi").unwrap(), "avi");
    }
    #[test]
    fn get_extension_mp4() {
        assert_eq!(
            get_extension("[Taka]_Fullmetal_Alchemist_(2009)_04_[720p][40F2A957].mp4").unwrap(),
            "mp4"
        );
        assert_eq!(get_extension("[Mobile Suit Gundam Seed Destiny HD REMASTER][07][Big5][720p][AVC_AAC][encoded by SEED].mp4").unwrap(),"mp4");
        assert_eq!(
            get_extension("「K」 Image Blu-ray WHITE & BLACK - Main (BD 1280x720 AVC AAC).mp4")
                .unwrap(),
            "mp4"
        );
        assert_eq!(
            get_extension("[[Zero-Raws] Shingeki no Kyojin - 05 (MBS 1280x720 x264 AAC).mp4")
                .unwrap(),
            "mp4"
        );
        assert_eq!(
            get_extension(
                "[Hakugetsu&Speed&MGRT][Dragon_Ball_Z_Battle_of_Gods][BDRIP][BIG5][1280x720].mp4"
            )
            .unwrap(),
            "mp4"
        );
        assert_eq!(
            get_extension("[Hakugetsu&MGRT][Evangelion 3.0 You Can (Not) Redo][480P][V0].mp4")
                .unwrap(),
            "mp4"
        );
        assert_eq!(get_extension("[TV-J] Kidou Senshi Gundam UC Unicorn - episode.02 [BD 1920x1080 h264+AAC(5.1ch JP+EN) +Sub(JP-EN-SP-FR-CH) Chap].mp4").unwrap(),"mp4");
        assert_eq!(
            get_extension("[Zero-Raws] Shingeki no Kyojin - 25 END (MBS 1280x720 x264 AAC).mp4")
                .unwrap(),
            "mp4"
        );
        assert_eq!(
            get_extension("Bakemonogatari - 01 (BD 1280x720 AVC AACx2).mp4").unwrap(),
            "mp4"
        );
        assert_eq!(
            get_extension("Evangelion The New Movie Q (BD 1280x720 AVC AACx2 [5.1+2.0]).mp4")
                .unwrap(),
            "mp4"
        );
        assert_eq!(
            get_extension("Kotonoha no Niwa (BD 1280x720 AVC AACx3 [5.1+2.0+2.0] Subx3).mp4")
                .unwrap(),
            "mp4"
        );
        assert_eq!(
            get_extension(
                "Queen's Blade Utsukushiki Toushi-tachi - OVA_01 (BD 1280x720 AVC AAC).mp4"
            )
            .unwrap(),
            "mp4"
        );
        assert_eq!(
            get_extension("[異域字幕組][漆黑的子彈][Black Bullet][11][1280x720][繁体].mp4")
                .unwrap(),
            "mp4"
        );
        assert_eq!(get_extension("[AoJiaoZero][Mangaka-san to Assistant-san to the Animation] 02 [BIG][X264_AAC][720P].mp4").unwrap(),"mp4");
        assert_eq!(
            get_extension(
                "[모에-Raws] Abarenbou Rikishi!! Matsutarou #04 (ABC 1280x720 x264 AAC).mp4"
            )
            .unwrap(),
            "mp4"
        );
        assert_eq!(
            get_extension("[바카-Raws] Nekomonogatari (Black) #1-4 (BS11 1280x720 x264 AAC).mp4")
                .unwrap(),
            "mp4"
        );
        assert_eq!(
            get_extension(
                "[SubDESU-H] Swing out Sisters Complete Version (720p x264 8bit AC3) [3ABD57E6].mp4"
            )
            .unwrap(),
            "mp4"
        );
        assert_eq!(
            get_extension("Cyborg 009 (1968) [TSHS] episode 06 [30C15D62].mp4").unwrap(),
            "mp4"
        );
        assert_eq!(
            get_extension("[5F] RWBY 14 Forever Fall Part 2 pt-BR.mp4").unwrap(),
            "mp4"
        );
    }
    #[test]
    fn get_extension_none() {
        assert!(get_extension("To_Aru_Kagaku_no_Railgun_13-15_[BD_1080p][AtsA]").is_none());
        assert!(
            get_extension("Hayate no Gotoku 2nd Season 24 (Blu-Ray 1080p) [Chihiro]").is_none()
        );
        assert!(
            get_extension("[BluDragon] Blue Submarine No.6 (DVD, R2, Dual Audio) V3").is_none()
        );
        assert!(get_extension("Chrono Crusade ep. 1-5").is_none());
        assert!(get_extension("[SFW]_Queen's_Blade_S2").is_none());
        assert!(get_extension("Evangelion_1.0_You_Are_[Not]_Alone_(1080p)_[@Home]").is_none());
        assert!(
            get_extension(
                "[Nubles] Space Battleship Yamato 2199 (2012) episode 18 (720p 8 bit AAC)[BA70BA9C]"
            )
            .is_none()
        );
        assert!(get_extension(
        "[Nubles] Space Battleship Yamato 2199 (2012) episode 18 (720p 10 bit AAC)[1F56D642]"
    )
    .is_none());
        assert!(get_extension("【MMZYSUB】★【Golden Time】[24（END）][GB][720P_MP4]").is_none());
        assert!(get_extension("Vol.01").is_none());
        assert!(get_extension(
        "Attack on Titan - Episode 3 - A Dim Light Amid Despair / Humanity's Comeback, Part 1"
    )
    .is_none());
        assert!(get_extension("DRAMAtical Murder Episode 1 - Data_01_Login").is_none());
        assert!(get_extension("[Coalgirls]_White_Album_1-13_(1280×720_Blu-Ray_FLAC)").is_none());
        assert!(
            get_extension(
                "[CH] Sword Art Online Extra Edition Dual Audio [BD 480p][10bitH.264+Vorbis]"
            )
            .is_none()
        );
        assert!(
            get_extension(
                "Episode 14 Ore no Imouto ga Konnani Kawaii Wake ga Nai. (saison 2) VOSTFR"
            )
            .is_none()
        );
        assert!(get_extension("Ep. 01 - The Boy in the Iceberg").is_none());
        assert!(
            get_extension("Byousoku 5 Centimeter [Blu-Ray][1920x1080 H.264][2.0ch AAC][SOFTSUBS]")
                .is_none()
        );
        assert!(get_extension("[LRL] 1001 Nights (1998) [DVD]").is_none());
        assert!(get_extension("[Anime").is_none());
        assert!(get_extension("Gekkan Shoujo Nozaki-kun [HorribleSubs] (1080p)").is_none());
        assert!(
            get_extension("[BM&T] Toradora! - 07v2 - Pool Opening [720p Hi10 ] [BD] [8F59F2BA]")
                .is_none()
        );
        assert!(get_extension("[EveTaku] AKB0048 Vol.03 - Making of Kibou-ni-Tsuite Music Video (BDRip 1080i H.264-Hi10P FLAC)[C09462E2]").is_none());
        assert!(
            get_extension("[DmonHiro] Magi - The Labyrinth Of Magic - Vol.1v2 (BD, 720p)")
                .is_none()
        );
        assert!(
            get_extension(
                "[tlacatlc6] Natsume Yuujinchou Shi Vol. 1v2 & Vol. 2 (BD 1280x720 x264 AAC)"
            )
            .is_none()
        );
        assert!(
            get_extension("[Tsundere] Hyouka - 01v2-04 [BDRip h264 1920x1080 10bit FLAC]")
                .is_none()
        );
        assert!(
            get_extension(
                "[Doki] Nogizaka Haruka no Himitsu - Purezza - 01v2-03v2 (1280x720 h264 AAC)"
            )
            .is_none()
        );
        assert!(
            get_extension(
                "Fairy Tail - S06E32 - Tartaros Arc Iron Fist of the Fire Dragon [Episode 83]"
            )
            .is_none()
        );
        assert!(get_extension("Noragami - S02E06 - What Must Be Done [Episode 6]").is_none());
        assert!(get_extension("[Harunatsu] Classroom Crisis - Vol.1 [BD 720p-AAC]").is_none());
        assert!(get_extension("[GS] Classroom Crisis Vol.1&2 (BD 1080p 10bit FLAC)").is_none());
        assert!(get_extension("[Infantjedi] Norn9 - Norn + Nonetto - 12").is_none());
        assert!(
            get_extension("Dragon_Ball_Z_Movies_8_&_10_[720p,BluRay,DTS,x264]_-_THORA").is_none()
        );
        assert!(get_extension("[HorribleSubs] Momokuri - 01+02 [720p]").is_none());
        assert!(get_extension("").is_none());
    }
    #[test]
    fn get_other_extension() {
        assert_eq!(
            get_extension("[Hard-Boiled FS]FullMetalAlchemist_09.rmvb").unwrap(),
            "rmvb"
        );
        assert_eq!(
        get_extension(
            "[Yuurisan-Subs]_Darker_than_Black_-_Gemini_of_the_Meteor_-_01v2_[65274FDE].patch.7z"
        )
        .unwrap(),
        "7z"
    );
        assert_eq!(
        get_extension(
            "[Yuurisan-Subs]_Darker_than_Black_-_Gemini_of_the_Meteor_-_01v2_[65274FDE].7z.patch.7z"
        )
        .unwrap(),
        "7z"
    );
    }

    #[test]
    fn mkv() {
        assert_eq!(
            remove_extension(
                "[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv"
            ),
            "[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD]"
        );
        assert_eq!(
            remove_extension("[ANBU]_Princess_Lover!_-_01_[2048A39A].mkv"),
            "[ANBU]_Princess_Lover!_-_01_[2048A39A]"
        );
        assert_eq!(
            remove_extension("[ANBU-Menclave]_Canaan_-_01_[1024x576_H.264_AAC][12F00E89].mkv"),
            "[ANBU-Menclave]_Canaan_-_01_[1024x576_H.264_AAC][12F00E89]"
        );
        assert_eq!(
            remove_extension("[ANBU-umai]_Haiyoru!_Nyaru-Ani_[596DD8E6].mkv"),
            "[ANBU-umai]_Haiyoru!_Nyaru-Ani_[596DD8E6]"
        );
        assert_eq!(
            remove_extension("[BakaWolf-m.3.3.w] Special A 01 (H.264) [C83164B9].mkv"),
            "[BakaWolf-m.3.3.w] Special A 01 (H.264) [C83164B9]"
        );
        assert_eq!(
            remove_extension(
                "[chibi-Doki] Seikon no Qwaser - 13v0 (Uncensored Director's Cut) [988DB090].mkv"
            ),
            "[chibi-Doki] Seikon no Qwaser - 13v0 (Uncensored Director's Cut) [988DB090]"
        );
        assert_eq!(
            remove_extension(
                "[Chihiro]_Kono_Aozora_ni_Yakusoku_Wo_10_v2_[DVD][h264][C83D206B].mkv"
            ),
            "[Chihiro]_Kono_Aozora_ni_Yakusoku_Wo_10_v2_[DVD][h264][C83D206B]"
        );
        assert_eq!(
            remove_extension("[Coalgirls]_Toradora_ED2_(704x480_DVD_AAC)_[3B65D1E6].mkv"),
            "[Coalgirls]_Toradora_ED2_(704x480_DVD_AAC)_[3B65D1E6]"
        );
        assert_eq!(
            remove_extension(
                "[Conclave-Mendoi]_Mobile_Suit_Gundam_00_S2_-_01v2_[1280x720_H.264_AAC][4863FBE8].mkv"
            ),
            "[Conclave-Mendoi]_Mobile_Suit_Gundam_00_S2_-_01v2_[1280x720_H.264_AAC][4863FBE8]"
        );
        assert_eq!(
            remove_extension("[Frostii]_Nodame_Cantabile_Finale_-_00_[73AD0735].mkv"),
            "[Frostii]_Nodame_Cantabile_Finale_-_00_[73AD0735]"
        );
        assert_eq!(
            remove_extension("[HorribleSubs] Tower of Druaga - Sword of Uruk - 04 [480p].mkv"),
            "[HorribleSubs] Tower of Druaga - Sword of Uruk - 04 [480p]"
        );
        assert_eq!(
            remove_extension(
                "[Juuni.Kokki]-(Les.12.Royaumes)-[Ep.24]-[x264+OGG]-[JAP+FR+Sub.FR]-[Chap]-[AzF].mkv"
            ),
            "[Juuni.Kokki]-(Les.12.Royaumes)-[Ep.24]-[x264+OGG]-[JAP+FR+Sub.FR]-[Chap]-[AzF]"
        );
        assert_eq!(
            remove_extension("[kito].Nazca.episode.01.DVDRip.[x264.He-aac.{Jpn}+Sub{Fr}].mkv"),
            "[kito].Nazca.episode.01.DVDRip.[x264.He-aac.{Jpn}+Sub{Fr}]"
        );
        assert_eq!(
            remove_extension(
                "[Lambda-Delta]_Umineko_no_Naku_Koro_ni_-_11_[848x480_H.264_AAC][943106AD].mkv"
            ),
            "[Lambda-Delta]_Umineko_no_Naku_Koro_ni_-_11_[848x480_H.264_AAC][943106AD]"
        );
        assert_eq!(
            remove_extension("[SS]_Kemono_no_Souja_Erin_-_12_(1280x720_h264)_[0F5F884F].mkv"),
            "[SS]_Kemono_no_Souja_Erin_-_12_(1280x720_h264)_[0F5F884F]"
        );
        assert_eq!(
            remove_extension("[UTW-TMD]_Summer_Wars_[BD][h264-720p][TrueHD5.1][9F311DAB].mkv"),
            "[UTW-TMD]_Summer_Wars_[BD][h264-720p][TrueHD5.1][9F311DAB]"
        );
        assert_eq!(
            remove_extension(
                "[ValdikSS]_First_Squad_The_Morment_Of_Truth_[720x576_h264_dvdscr_eng_hardsub].mkv"
            ),
            "[ValdikSS]_First_Squad_The_Morment_Of_Truth_[720x576_h264_dvdscr_eng_hardsub]"
        );
        assert_eq!(
            remove_extension(
                "Code_Geass_R2_TV_[20_of_25]_[ru_jp]_[HDTV]_[Varies_&_Cuba77_&_AnimeReactor_RU].mkv"
            ),
            "Code_Geass_R2_TV_[20_of_25]_[ru_jp]_[HDTV]_[Varies_&_Cuba77_&_AnimeReactor_RU]"
        );
        assert_eq!(
            remove_extension(
                "Evangelion_1.11_You_Are_(Not)_Alone_(2009)_[1080p,BluRay,x264,DTS-ES]_-_THORA.mkv"
            ),
            "Evangelion_1.11_You_Are_(Not)_Alone_(2009)_[1080p,BluRay,x264,DTS-ES]_-_THORA"
        );
        assert_eq!(
            remove_extension(
                "Evangelion_1.11_You_Are_(Not)_Alone_[1080p,BluRay,x264,DTS-ES]_-_THORA.mkv"
            ),
            "Evangelion_1.11_You_Are_(Not)_Alone_[1080p,BluRay,x264,DTS-ES]_-_THORA"
        );
        assert_eq!(
            remove_extension("Eve no Jikan 2 [88F4F7F0].mkv"),
            "Eve no Jikan 2 [88F4F7F0]"
        );
        assert_eq!(
            remove_extension("Gin'iro_no_Kami_no_Agito_(2006)_[1080p,BluRay,x264,DTS]_-_THORA.mkv"),
            "Gin'iro_no_Kami_no_Agito_(2006)_[1080p,BluRay,x264,DTS]_-_THORA"
        );
        assert_eq!(
            remove_extension(
                "Magical Girl Lyrical Nanoha A's - 01.DVD[H264.AAC][DGz][7A8A7769].mkv"
            ),
            "Magical Girl Lyrical Nanoha A's - 01.DVD[H264.AAC][DGz][7A8A7769]"
        );
        assert_eq!(
            remove_extension(
                "Mobile_Suit_Gundam_00_Season_2_Ep07_A_Reunion_and_a_Parting_[1080p,BluRay,x264]_-_THORA.mkv"
            ),
            "Mobile_Suit_Gundam_00_Season_2_Ep07_A_Reunion_and_a_Parting_[1080p,BluRay,x264]_-_THORA"
        );
        assert_eq!(
            remove_extension("Noein_[01_of_24]_[ru_jp]_[bodlerov_&_torrents_ru].mkv"),
            "Noein_[01_of_24]_[ru_jp]_[bodlerov_&_torrents_ru]"
        );
        assert_eq!(
            remove_extension("ponyo_on_the_cliff_by_the_sea[h264.dts][niizk].mkv"),
            "ponyo_on_the_cliff_by_the_sea[h264.dts][niizk]"
        );
        assert_eq!(
            remove_extension(
                "[Seto_Otaku]_AIKa_ZERO_OVA_-_01_[BD][1920x1080_H264-Flac][6730D40A].mkv"
            ),
            "[Seto_Otaku]_AIKa_ZERO_OVA_-_01_[BD][1920x1080_H264-Flac][6730D40A]"
        );
        assert_eq!(
            remove_extension("[a4e]R.O.D_the_TV_01[divx5.2.1].mkv"),
            "[a4e]R.O.D_the_TV_01[divx5.2.1]"
        );
        assert_eq!(
            remove_extension(
                "Ghost_in_the_Shell_Stand_Alone_Complex_2nd_GIG_Ep05v2_EXCAVATION_[720p,HDTV,x264,AAC_5.1]_-_THORA.mkv"
            ),
            "Ghost_in_the_Shell_Stand_Alone_Complex_2nd_GIG_Ep05v2_EXCAVATION_[720p,HDTV,x264,AAC_5.1]_-_THORA"
        );
        assert_eq!(
            remove_extension(
                "Ghost_in_the_Shell_Stand_Alone_Complex_2nd_GIG_Ep06_Pu239_[720p,HDTV,x264,AAC_5.1]_-_THORA.mkv"
            ),
            "Ghost_in_the_Shell_Stand_Alone_Complex_2nd_GIG_Ep06_Pu239_[720p,HDTV,x264,AAC_5.1]_-_THORA"
        );
        assert_eq!(
            remove_extension(
                "Fate_Stay_Night_Ep05_The_Two_Magi_Part1_[720p,BluRay,x264]_-_THORA.mkv"
            ),
            "Fate_Stay_Night_Ep05_The_Two_Magi_Part1_[720p,BluRay,x264]_-_THORA"
        );
        assert_eq!(
            remove_extension("[RaX]Mezzo(DSA)_-_05_-_[x264_ogg]_[585d9971].mkv"),
            "[RaX]Mezzo(DSA)_-_05_-_[x264_ogg]_[585d9971]"
        );
        assert_eq!(
            remove_extension("[AKH-SWE]_Fullmetal_Alchemist_(2009)_02v2_[H.264.AAC][7B2C5E8B].mkv"),
            "[AKH-SWE]_Fullmetal_Alchemist_(2009)_02v2_[H.264.AAC][7B2C5E8B]"
        );
        assert_eq!(
            remove_extension("[FuktLogik][Sayonara_Zetsubou_Sensei][01][DVDRip][x264_AC3].mkv"),
            "[FuktLogik][Sayonara_Zetsubou_Sensei][01][DVDRip][x264_AC3]"
        );
        assert_eq!(
            remove_extension("[Ayu]_Kiddy_Grade_2_-_Pilot_[H264_AC3][650B731B].mkv"),
            "[Ayu]_Kiddy_Grade_2_-_Pilot_[H264_AC3][650B731B]"
        );
        assert_eq!(
            remove_extension(
                "[Darksoul-Subs] Tatakau Shisho - The Book of Bantorra [848x480 XVID_MP3].mkv"
            ),
            "[Darksoul-Subs] Tatakau Shisho - The Book of Bantorra [848x480 XVID_MP3]"
        );
        assert_eq!(
            remove_extension(
                "[ACX]Neon_Genesis_Evangelion_-_Platinum_-_06_-_Showdown_in_Tokyo_3_[SaintDeath]_[CBDB8577].mkv"
            ),
            "[ACX]Neon_Genesis_Evangelion_-_Platinum_-_06_-_Showdown_in_Tokyo_3_[SaintDeath]_[CBDB8577]"
        );
        assert_eq!(
            remove_extension("[Himatsubushi]_Sora_no_Woto_-_01_-_H264_-_720p_-_E83AD672.mkv"),
            "[Himatsubushi]_Sora_no_Woto_-_01_-_H264_-_720p_-_E83AD672"
        );
        assert_eq!(
            remove_extension(
                "[EroGaKi-Team]_Nurse_Witch_Komugi-chan_Magikarte_02.5_[902BB314].mkv"
            ),
            "[EroGaKi-Team]_Nurse_Witch_Komugi-chan_Magikarte_02.5_[902BB314]"
        );
        assert_eq!(
            remove_extension("Ookiku Furikabutte S2 - 09 (Central Anime) [BD841253].mkv"),
            "Ookiku Furikabutte S2 - 09 (Central Anime) [BD841253]"
        );
        assert_eq!(
            remove_extension("[HorribleSubs] HEROMAN - 10_(XviD_AnimeSenshi).mkv"),
            "[HorribleSubs] HEROMAN - 10_(XviD_AnimeSenshi)"
        );
        assert_eq!(
            remove_extension("Detective Conan - 316-317 [DCTP][2411959B].mkv"),
            "Detective Conan - 316-317 [DCTP][2411959B]"
        );
        assert_eq!(
            remove_extension("[N LogN Fansubs] Angel Beats (9).mkv"),
            "[N LogN Fansubs] Angel Beats (9)"
        );
        assert_eq!(
            remove_extension("Juuousei_-_01_[Black_Sheep][HDTV_H264_AAC][803DA487].mkv"),
            "Juuousei_-_01_[Black_Sheep][HDTV_H264_AAC][803DA487]"
        );
        assert_eq!(
            remove_extension("[gg]_Kimi_ni_Todoke_2nd_Season_-_00_[BF735BC4].mkv"),
            "[gg]_Kimi_ni_Todoke_2nd_Season_-_00_[BF735BC4]"
        );
        assert_eq!(
            remove_extension("K-ON!_Ep03_Training!_[1080p,BluRay,x264]_-_THORA.mkv"),
            "K-ON!_Ep03_Training!_[1080p,BluRay,x264]_-_THORA"
        );
        assert_eq!(
            remove_extension("K-ON!!_Ep08_Career_Plan!_[1080p,BluRay,x264]_-_THORA.mkv"),
            "K-ON!!_Ep08_Career_Plan!_[1080p,BluRay,x264]_-_THORA"
        );
        assert_eq!(
            remove_extension(
                "[E-HARO Raws] Kore wa Zombie desu ka - 03 (TV 1280x720 h264 AAC) [888E4991].mkv"
            ),
            "[E-HARO Raws] Kore wa Zombie desu ka - 03 (TV 1280x720 h264 AAC) [888E4991]"
        );
        assert_eq!(
            remove_extension("[Edomae Subs] Kore wa Zombie desu ka  Episode 2.mkv"),
            "[Edomae Subs] Kore wa Zombie desu ka  Episode 2"
        );
        assert_eq!(
            remove_extension("5_centimeters_per_second[1904x1072.h264.flac][niizk].mkv"),
            "5_centimeters_per_second[1904x1072.h264.flac][niizk]"
        );
        assert_eq!(
            remove_extension("[Yoroshiku]_009-1_-_02_(H264)_[36D2444D].mkv"),
            "[Yoroshiku]_009-1_-_02_(H264)_[36D2444D]"
        );
        assert_eq!(
            remove_extension("After War Gundam X - 1x03 - My Mount is Fierce!.mkv"),
            "After War Gundam X - 1x03 - My Mount is Fierce!"
        );
        assert_eq!(
            remove_extension("[HorribleSubs] The World God Only Knows 2 - 03 [720p].mkv"),
            "[HorribleSubs] The World God Only Knows 2 - 03 [720p]"
        );
        assert_eq!(
            remove_extension(
                "Macross Frontier - Sayonara no Tsubasa (Central Anime, 720p) [46B35E25].mkv"
            ),
            "Macross Frontier - Sayonara no Tsubasa (Central Anime, 720p) [46B35E25]"
        );
        assert_eq!(
            remove_extension("[FFF] Red Data Girl - 10v0 [29EA865B].mkv"),
            "[FFF] Red Data Girl - 10v0 [29EA865B]"
        );
        assert_eq!(
            remove_extension("[CMS] Magical☆Star Kanon 100% OVA[DVD][E9F43685].mkv"),
            "[CMS] Magical☆Star Kanon 100% OVA[DVD][E9F43685]"
        );
        assert_eq!(
            remove_extension("[Doremi].Ro-Kyu-Bu!.SS.01.[C1B5CE5D].mkv"),
            "[Doremi].Ro-Kyu-Bu!.SS.01.[C1B5CE5D]"
        );
        assert_eq!(
            remove_extension(
                "[Raizel] Persona 4 The Animation Episode 13 - A Stormy Summer Vacation Part 1  [BD_1080p_Dual_Audio_FLAC_Hi10p][8A45634B].mkv"
            ),
            "[Raizel] Persona 4 The Animation Episode 13 - A Stormy Summer Vacation Part 1  [BD_1080p_Dual_Audio_FLAC_Hi10p][8A45634B]"
        );
        assert_eq!(
            remove_extension(
                "[Hien] Kotoura-san - Special Short Anime 'Haruka's Room' - 01 [BD 1080p H.264 10-bit AAC][6B6BE015].mkv"
            ),
            "[Hien] Kotoura-san - Special Short Anime 'Haruka's Room' - 01 [BD 1080p H.264 10-bit AAC][6B6BE015]"
        );
        assert_eq!(
            remove_extension("[R-R] Diebuster.EP1 (720p.Hi10p.AC3) [82E36A36].mkv"),
            "[R-R] Diebuster.EP1 (720p.Hi10p.AC3) [82E36A36]"
        );
        assert_eq!(
            remove_extension(
                "Aim_For_The_Top!_Gunbuster-ep1.BD(H264.FLAC.10bit)[KAA][69ECCDCF].mkv"
            ),
            "Aim_For_The_Top!_Gunbuster-ep1.BD(H264.FLAC.10bit)[KAA][69ECCDCF]"
        );
        assert_eq!(
            remove_extension("[Rakuda].Gift.~eternal.rainbow~.01.dvd.h.264.vorbis.mkv"),
            "[Rakuda].Gift.~eternal.rainbow~.01.dvd.h.264.vorbis"
        );
        assert_eq!(
            remove_extension(
                "[Jumonji-Giri]_[Shinsen-Subs][ASF]_D.C.II_Da_Capo_II_Ep01_(a1fc58a7).mkv"
            ),
            "[Jumonji-Giri]_[Shinsen-Subs][ASF]_D.C.II_Da_Capo_II_Ep01_(a1fc58a7)"
        );
        assert_eq!(
            remove_extension("[52wy][SlamDunk][001][Jpn_Chs_Cht][x264_aac][DVDRip][7FE2C873].mkv"),
            "[52wy][SlamDunk][001][Jpn_Chs_Cht][x264_aac][DVDRip][7FE2C873]"
        );
        assert_eq!(
            remove_extension("[Commie] Last Exile ~Fam, The Silver Wing~ - 13 [AFF9E530].mkv"),
            "[Commie] Last Exile ~Fam, The Silver Wing~ - 13 [AFF9E530]"
        );
        assert_eq!(
            remove_extension("[UTW]_Fate_Zero_-_01_[BD][h264-720p_AC3][02A0491D].mkv"),
            "[UTW]_Fate_Zero_-_01_[BD][h264-720p_AC3][02A0491D]"
        );
        assert_eq!(
            remove_extension(
                "[UTW-THORA] Evangelion 3.33 You Can (Not) Redo [BD][1080p,x264,flac][F2060CF5].mkv"
            ),
            "[UTW-THORA] Evangelion 3.33 You Can (Not) Redo [BD][1080p,x264,flac][F2060CF5]"
        );
        assert_eq!(
            remove_extension(
                "Evangelion Shin Gekijouban Q (BDrip 1920x1080 x264 FLACx2 5.1ch)-ank.mkv"
            ),
            "Evangelion Shin Gekijouban Q (BDrip 1920x1080 x264 FLACx2 5.1ch)-ank"
        );
        assert_eq!(
            remove_extension(
                "Howl's_Moving_Castle_(2004)_[1080p,BluRay,flac,dts,x264]_-_THORA v2.mkv"
            ),
            "Howl's_Moving_Castle_(2004)_[1080p,BluRay,flac,dts,x264]_-_THORA v2"
        );
        assert_eq!(
            remove_extension(
                "[FFF] Futsuu no Joshikousei ga [Locodol] Yatte Mita. - 01 [BAD09C76].mkv"
            ),
            "[FFF] Futsuu no Joshikousei ga [Locodol] Yatte Mita. - 01 [BAD09C76]"
        );
        assert_eq!(
            remove_extension("[Asenshi] Rozen Maiden 3 - PV [CA57F300].mkv"),
            "[Asenshi] Rozen Maiden 3 - PV [CA57F300]"
        );
        assert_eq!(
            remove_extension("Mary Bell (DVD) - 01v2 [h-b].mkv"),
            "Mary Bell (DVD) - 01v2 [h-b]"
        );
        assert_eq!(
            remove_extension("Mary Bell (DVD) - 02 [h-b].mkv"),
            "Mary Bell (DVD) - 02 [h-b]"
        );
        assert_eq!(
            remove_extension("The Irregular at Magic High School - S01E01- Enrollment Part I.mkv"),
            "The Irregular at Magic High School - S01E01- Enrollment Part I"
        );
        assert_eq!(
            remove_extension("[Mezashite] Aikatsu! ‒ 100 [D035A39F].mkv"),
            "[Mezashite] Aikatsu! ‒ 100 [D035A39F]"
        );
        assert_eq!(
            remove_extension("[UTW]_Accel_World_-_EX01_[BD][h264-720p_AAC][3E56EE18].mkv"),
            "[UTW]_Accel_World_-_EX01_[BD][h264-720p_AAC][3E56EE18]"
        );
        assert_eq!(
            remove_extension("[HorribleSubs] Tsukimonogatari - (01-04) [1080p].mkv"),
            "[HorribleSubs] Tsukimonogatari - (01-04) [1080p]"
        );
        assert_eq!(
            remove_extension("[Urusai]_Bokura_Ga_Ita_01_[DVD_h264_AC3]_[BFCE1627][Fixed].mkv"),
            "[Urusai]_Bokura_Ga_Ita_01_[DVD_h264_AC3]_[BFCE1627][Fixed]"
        );
        assert_eq!(
            remove_extension(
                "[Coalgirls]_Bakemonogatari_OP4a_(1280x720_Blu-Ray_FLAC)_[327A2375].mkv"
            ),
            "[Coalgirls]_Bakemonogatari_OP4a_(1280x720_Blu-Ray_FLAC)_[327A2375]"
        );
        assert_eq!(
            remove_extension("[Ruri]No.6 01 [720p][H264][A956075C].mkv"),
            "[Ruri]No.6 01 [720p][H264][A956075C]"
        );
        assert_eq!(
            remove_extension("EvoBot.[Watakushi]_Akuma_no_Riddle_-_01v2_[720p][69A307A2].mkv"),
            "EvoBot.[Watakushi]_Akuma_no_Riddle_-_01v2_[720p][69A307A2]"
        );
        assert_eq!(
            remove_extension("01 - Land of Visible Pain.mkv"),
            "01 - Land of Visible Pain"
        );
        assert_eq!(
            remove_extension(
                "[Zom-B] Machine-Doll wa Kizutsukanai - 01 - Facing ''Cannibal Candy'' I (kuroi, FFF remux) [B99C8DED].mkv"
            ),
            "[Zom-B] Machine-Doll wa Kizutsukanai - 01 - Facing ''Cannibal Candy'' I (kuroi, FFF remux) [B99C8DED]"
        );
        assert_eq!(
            remove_extension("The iDOLM@STER 765 Pro to Iu Monogatari.mkv"),
            "The iDOLM@STER 765 Pro to Iu Monogatari"
        );
        assert_eq!(
            remove_extension("[Coalgirls]_Fate_Zero_OVA3.5_(1280x720_Blu-ray_FLAC)_[5F5AD026].mkv"),
            "[Coalgirls]_Fate_Zero_OVA3.5_(1280x720_Blu-ray_FLAC)_[5F5AD026]"
        );
        assert_eq!(
            remove_extension("[GrimRipper] Koi Kaze [Dual Audio] Ep01 (c13cefe0).mkv"),
            "[GrimRipper] Koi Kaze [Dual Audio] Ep01 (c13cefe0)"
        );
        assert_eq!(
            remove_extension("[HorribleSubs] Gintama - 111C [1080p].mkv"),
            "[HorribleSubs] Gintama - 111C [1080p]"
        );
        assert_eq!(
            remove_extension("[SpoonSubs]_Hidamari_Sketch_x365_-_04.1_(DVD)[B6CE8458].mkv"),
            "[SpoonSubs]_Hidamari_Sketch_x365_-_04.1_(DVD)[B6CE8458]"
        );
        assert_eq!(
            remove_extension(
                "[Hatsuyuki]_Kuroko_no_Basuke_S3_-_01_(51)_[720p][10bit][619C57A0].mkv"
            ),
            "[Hatsuyuki]_Kuroko_no_Basuke_S3_-_01_(51)_[720p][10bit][619C57A0]"
        );
        assert_eq!(
            remove_extension("[Elysium]Sora.no.Woto.EP07.5(BD.720p.AAC)[C37580F8].mkv"),
            "[Elysium]Sora.no.Woto.EP07.5(BD.720p.AAC)[C37580F8]"
        );
        assert_eq!(
            remove_extension(
                "[Zurako] Sora no Woto - 07.5 - Drinking Party - Fortress Battle (BD 1080p AAC) [F7DF16F7].mkv"
            ),
            "[Zurako] Sora no Woto - 07.5 - Drinking Party - Fortress Battle (BD 1080p AAC) [F7DF16F7]"
        );
        assert_eq!(
            remove_extension("[Hiryuu] Maji de Watashi ni Koi Shinasai!! - 02 [720].mkv"),
            "[Hiryuu] Maji de Watashi ni Koi Shinasai!! - 02 [720]"
        );
        assert_eq!(
            remove_extension(
                "[Kira-Fansub] Uchuu no Stellvia ep 14 (BD H264 1280x960 24fps AAC) [06EE7355].mkv"
            ),
            "[Kira-Fansub] Uchuu no Stellvia ep 14 (BD H264 1280x960 24fps AAC) [06EE7355]"
        );
        assert_eq!(
            remove_extension(
                "[ANE] Yosuga no Sora - Ep01 Preview (Yorihime ver) [BDRip 1080p x264 FLAC].mkv"
            ),
            "[ANE] Yosuga no Sora - Ep01 Preview (Yorihime ver) [BDRip 1080p x264 FLAC]"
        );
        assert_eq!(
            remove_extension("[B-G_&_m.3.3.w]_Myself_Yourself_12.DVD(H.264_DD2.0)_[CB2B37F1].mkv"),
            "[B-G_&_m.3.3.w]_Myself_Yourself_12.DVD(H.264_DD2.0)_[CB2B37F1]"
        );
        assert_eq!(
            remove_extension("The.Animatrix.08.A.Detective.Story.720p.BluRay.DTS.x264-ESiR.mkv"),
            "The.Animatrix.08.A.Detective.Story.720p.BluRay.DTS.x264-ESiR"
        );
        assert_eq!(
            remove_extension(
                "[DmonHiro] Oreshura #01v2 - The Start Of High School Life Is A War Zone [BD, 720p] [211375E6].mkv"
            ),
            "[DmonHiro] Oreshura #01v2 - The Start Of High School Life Is A War Zone [BD, 720p] [211375E6]"
        );
        assert_eq!(
            remove_extension(
                "[NinjaPanda] Tiger & Bunny #01 All's well that ends well. (v3, 1080p Hi10P, DA AAC) [4A9AB85F].mkv"
            ),
            "[NinjaPanda] Tiger & Bunny #01 All's well that ends well. (v3, 1080p Hi10P, DA AAC) [4A9AB85F]"
        );
        assert_eq!(
            remove_extension(
                "Neko no Ongaeshi - [HQR.remux-DualAudio][NTV.1280x692.h264](0CDC2145).mkv"
            ),
            "Neko no Ongaeshi - [HQR.remux-DualAudio][NTV.1280x692.h264](0CDC2145)"
        );
        assert_eq!(
            remove_extension("[ReDone] Memories Off 3.5 - 04 (DVD 10-bit).mkv"),
            "[ReDone] Memories Off 3.5 - 04 (DVD 10-bit)"
        );
        assert_eq!(
            remove_extension(
                "[FFF] Seirei Tsukai no Blade Dance - SP01 [BD][720p-AAC][F1FF8588].mkv"
            ),
            "[FFF] Seirei Tsukai no Blade Dance - SP01 [BD][720p-AAC][F1FF8588]"
        );
        assert_eq!(
            remove_extension(
                "[Hatsuyuki] Dragon Ball Kai (2014) - 002 (100) [1280x720][DD66AFB7].mkv"
            ),
            "[Hatsuyuki] Dragon Ball Kai (2014) - 002 (100) [1280x720][DD66AFB7]"
        );
        assert_eq!(
            remove_extension("[Deep] Tegami Bachi (REVERSE) - Letter Bee - 29 (04) [5203142B].mkv"),
            "[Deep] Tegami Bachi (REVERSE) - Letter Bee - 29 (04) [5203142B]"
        );
        assert_eq!(
            remove_extension("[FFF] Love Live! The School Idol Movie - PV [D1A15D2C].mkv"),
            "[FFF] Love Live! The School Idol Movie - PV [D1A15D2C]"
        );
        assert_eq!(
            remove_extension(
                "[Nishi-Taku] Tamayura ~graduation photo~ Movie Part 1 [BD][720p][98965607].mkv"
            ),
            "[Nishi-Taku] Tamayura ~graduation photo~ Movie Part 1 [BD][720p][98965607]"
        );
        assert_eq!(
            remove_extension("[TardRaws] 0 [640x360].mkv"),
            "[TardRaws] 0 [640x360]"
        );
        assert_eq!(
            remove_extension(
                "[Hatsuyuki-Kaitou]_Fairy_Tail_2_-_52_(227)_[720p][10bit][9DF6B8D5].mkv"
            ),
            "[Hatsuyuki-Kaitou]_Fairy_Tail_2_-_52_(227)_[720p][10bit][9DF6B8D5]"
        );
        assert_eq!(
            remove_extension(
                "[FBI] Baby Princess 3D Paradise Love 01v0 [BD][720p-AAC][457CC066].mkv"
            ),
            "[FBI] Baby Princess 3D Paradise Love 01v0 [BD][720p-AAC][457CC066]"
        );
        assert_eq!(
            remove_extension("[NamaeNai] Hidamari Sketch x365 - 09a (DVD) [49874745].mkv"),
            "[NamaeNai] Hidamari Sketch x365 - 09a (DVD) [49874745]"
        );
        assert_eq!(
            remove_extension("[FaggotryRaws] Bakuman - 01 (NHK E 848x480).mkv"),
            "[FaggotryRaws] Bakuman - 01 (NHK E 848x480)"
        );
        assert_eq!(
            remove_extension("Dragon.Ball.KAI.-.01.-.1080p.BluRay.x264.DHD.mkv"),
            "Dragon.Ball.KAI.-.01.-.1080p.BluRay.x264.DHD"
        );
        assert_eq!(
            remove_extension("[AnimeRG] Ushio to Tora (TV) - 02 [720p] [Xcelent].mkv"),
            "[AnimeRG] Ushio to Tora (TV) - 02 [720p] [Xcelent]"
        );
        assert_eq!(
            remove_extension(
                "[(´• ω •`)] Nintama Rantarou - S23E1821 - Buddhist Priest-sama is a Ninja.mkv"
            ),
            "[(´• ω •`)] Nintama Rantarou - S23E1821 - Buddhist Priest-sama is a Ninja"
        );
        assert_eq!(
            remove_extension("[Judas] Aharen-san wa Hakarenai - S01E06v2.mkv"),
            "[Judas] Aharen-san wa Hakarenai - S01E06v2"
        );
        assert_eq!(
            remove_extension(
                "[0x539] Somali and the Forest Spirit - S01E01 (WEB 1080p Hi10P AAC) [BB7C6531].mkv"
            ),
            "[0x539] Somali and the Forest Spirit - S01E01 (WEB 1080p Hi10P AAC) [BB7C6531]"
        );
    }

    #[test]
    fn avi() {
        assert_eq!(
            remove_extension("[DB]_Bleach_225_[C63D149C].avi"),
            "[DB]_Bleach_225_[C63D149C]"
        );
        assert_eq!(
            remove_extension("[KAF-TEAM]_One_Piece_Movie_9_vostfr_HD.avi"),
            "[KAF-TEAM]_One_Piece_Movie_9_vostfr_HD"
        );
        assert_eq!(
            remove_extension("[RNA]_Sakura_Taisen_New_York_NY_Ep_2_[1590D378].avi"),
            "[RNA]_Sakura_Taisen_New_York_NY_Ep_2_[1590D378]"
        );
        assert_eq!(
            remove_extension("[Ayako]_Infinite_Stratos_-_IS_-_01v2_[XVID][400p][29675B71].avi"),
            "[Ayako]_Infinite_Stratos_-_IS_-_01v2_[XVID][400p][29675B71]"
        );
        assert_eq!(remove_extension("Juuni.Kokki.Ep.5.avi"), "Juuni.Kokki.Ep.5");
        assert_eq!(remove_extension("Juuni Kokki Ep.5.avi"), "Juuni Kokki Ep.5");
        assert_eq!(
            remove_extension("[Keroro].148.[Xvid.mp3].[FE68D5F1].avi"),
            "[Keroro].148.[Xvid.mp3].[FE68D5F1]"
        );
        assert_eq!(
            remove_extension("[Triad]_Today_in_Class_5-2_-_04.avi"),
            "[Triad]_Today_in_Class_5-2_-_04"
        );
        assert_eq!(
            remove_extension("__BLUE DROP 10 (1).avi"),
            "__BLUE DROP 10 (1)"
        );
        assert_eq!(
            remove_extension("37 [Ruberia]_Death_Note_-_37v2_[FINAL]_[XviD][6FA7D273].avi"),
            "37 [Ruberia]_Death_Note_-_37v2_[FINAL]_[XviD][6FA7D273]"
        );
        assert_eq!(
            remove_extension(
                "[FB] Crayon Shin-Chan Movie 2 The Secret of Buri Buri Kingdom [DivX5 AC3] 1994 [852X480] V2.avi"
            ),
            "[FB] Crayon Shin-Chan Movie 2 The Secret of Buri Buri Kingdom [DivX5 AC3] 1994 [852X480] V2"
        );
        assert_eq!(
            remove_extension("[Shinsen-Subs]_Macross_Frontier_-_01b_[4D5EC315].avi"),
            "[Shinsen-Subs]_Macross_Frontier_-_01b_[4D5EC315]"
        );
        assert_eq!(
            remove_extension("[KLF]_D.Gray-man_04V2.avi"),
            "[KLF]_D.Gray-man_04V2"
        );
    }

    #[test]
    fn mp4() {
        assert_eq!(
            remove_extension("[Taka]_Fullmetal_Alchemist_(2009)_04_[720p][40F2A957].mp4"),
            "[Taka]_Fullmetal_Alchemist_(2009)_04_[720p][40F2A957]"
        );
        assert_eq!(
            remove_extension(
                "[Mobile Suit Gundam Seed Destiny HD REMASTER][07][Big5][720p][AVC_AAC][encoded by SEED].mp4"
            ),
            "[Mobile Suit Gundam Seed Destiny HD REMASTER][07][Big5][720p][AVC_AAC][encoded by SEED]"
        );
        assert_eq!(
            remove_extension("「K」 Image Blu-ray WHITE & BLACK - Main (BD 1280x720 AVC AAC).mp4"),
            "「K」 Image Blu-ray WHITE & BLACK - Main (BD 1280x720 AVC AAC)"
        );
        assert_eq!(
            remove_extension("[[Zero-Raws] Shingeki no Kyojin - 05 (MBS 1280x720 x264 AAC).mp4"),
            "[[Zero-Raws] Shingeki no Kyojin - 05 (MBS 1280x720 x264 AAC)"
        );
        assert_eq!(
            remove_extension(
                "[Hakugetsu&Speed&MGRT][Dragon_Ball_Z_Battle_of_Gods][BDRIP][BIG5][1280x720].mp4"
            ),
            "[Hakugetsu&Speed&MGRT][Dragon_Ball_Z_Battle_of_Gods][BDRIP][BIG5][1280x720]"
        );
        assert_eq!(
            remove_extension("[Hakugetsu&MGRT][Evangelion 3.0 You Can (Not) Redo][480P][V0].mp4"),
            "[Hakugetsu&MGRT][Evangelion 3.0 You Can (Not) Redo][480P][V0]"
        );
        assert_eq!(
            remove_extension(
                "[TV-J] Kidou Senshi Gundam UC Unicorn - episode.02 [BD 1920x1080 h264+AAC(5.1ch JP+EN) +Sub(JP-EN-SP-FR-CH) Chap].mp4"
            ),
            "[TV-J] Kidou Senshi Gundam UC Unicorn - episode.02 [BD 1920x1080 h264+AAC(5.1ch JP+EN) +Sub(JP-EN-SP-FR-CH) Chap]"
        );
        assert_eq!(
            remove_extension("[Zero-Raws] Shingeki no Kyojin - 25 END (MBS 1280x720 x264 AAC).mp4"),
            "[Zero-Raws] Shingeki no Kyojin - 25 END (MBS 1280x720 x264 AAC)"
        );
        assert_eq!(
            remove_extension("Bakemonogatari - 01 (BD 1280x720 AVC AACx2).mp4"),
            "Bakemonogatari - 01 (BD 1280x720 AVC AACx2)"
        );
        assert_eq!(
            remove_extension("Evangelion The New Movie Q (BD 1280x720 AVC AACx2 [5.1+2.0]).mp4"),
            "Evangelion The New Movie Q (BD 1280x720 AVC AACx2 [5.1+2.0])"
        );
        assert_eq!(
            remove_extension("Kotonoha no Niwa (BD 1280x720 AVC AACx3 [5.1+2.0+2.0] Subx3).mp4"),
            "Kotonoha no Niwa (BD 1280x720 AVC AACx3 [5.1+2.0+2.0] Subx3)"
        );
        assert_eq!(
            remove_extension(
                "Queen's Blade Utsukushiki Toushi-tachi - OVA_01 (BD 1280x720 AVC AAC).mp4"
            ),
            "Queen's Blade Utsukushiki Toushi-tachi - OVA_01 (BD 1280x720 AVC AAC)"
        );
        assert_eq!(
            remove_extension("[異域字幕組][漆黑的子彈][Black Bullet][11][1280x720][繁体].mp4"),
            "[異域字幕組][漆黑的子彈][Black Bullet][11][1280x720][繁体]"
        );
        assert_eq!(
            remove_extension(
                "[AoJiaoZero][Mangaka-san to Assistant-san to the Animation] 02 [BIG][X264_AAC][720P].mp4"
            ),
            "[AoJiaoZero][Mangaka-san to Assistant-san to the Animation] 02 [BIG][X264_AAC][720P]"
        );
        assert_eq!(
            remove_extension(
                "[모에-Raws] Abarenbou Rikishi!! Matsutarou #04 (ABC 1280x720 x264 AAC).mp4"
            ),
            "[모에-Raws] Abarenbou Rikishi!! Matsutarou #04 (ABC 1280x720 x264 AAC)"
        );
        assert_eq!(
            remove_extension(
                "[바카-Raws] Nekomonogatari (Black) #1-4 (BS11 1280x720 x264 AAC).mp4"
            ),
            "[바카-Raws] Nekomonogatari (Black) #1-4 (BS11 1280x720 x264 AAC)"
        );
        assert_eq!(
            remove_extension(
                "[SubDESU-H] Swing out Sisters Complete Version (720p x264 8bit AC3) [3ABD57E6].mp4"
            ),
            "[SubDESU-H] Swing out Sisters Complete Version (720p x264 8bit AC3) [3ABD57E6]"
        );
        assert_eq!(
            remove_extension("Cyborg 009 (1968) [TSHS] episode 06 [30C15D62].mp4"),
            "Cyborg 009 (1968) [TSHS] episode 06 [30C15D62]"
        );
        assert_eq!(
            remove_extension("[5F] RWBY 14 Forever Fall Part 2 pt-BR.mp4"),
            "[5F] RWBY 14 Forever Fall Part 2 pt-BR"
        );
    }

    #[test]
    fn no_extension() {
        assert_eq!(
            remove_extension("To_Aru_Kagaku_no_Railgun_13-15_[BD_1080p][AtsA]"),
            "To_Aru_Kagaku_no_Railgun_13-15_[BD_1080p][AtsA]"
        );
        assert_eq!(
            remove_extension("Hayate no Gotoku 2nd Season 24 (Blu-Ray 1080p) [Chihiro]"),
            "Hayate no Gotoku 2nd Season 24 (Blu-Ray 1080p) [Chihiro]"
        );
        assert_eq!(
            remove_extension("[BluDragon] Blue Submarine No.6 (DVD, R2, Dual Audio) V3"),
            "[BluDragon] Blue Submarine No.6 (DVD, R2, Dual Audio) V3"
        );
        assert_eq!(
            remove_extension("Chrono Crusade ep. 1-5"),
            "Chrono Crusade ep. 1-5"
        );
        assert_eq!(
            remove_extension("[SFW]_Queen's_Blade_S2"),
            "[SFW]_Queen's_Blade_S2"
        );
        assert_eq!(
            remove_extension("Evangelion_1.0_You_Are_[Not]_Alone_(1080p)_[@Home]"),
            "Evangelion_1.0_You_Are_[Not]_Alone_(1080p)_[@Home]"
        );
        assert_eq!(
            remove_extension(
                "[Nubles] Space Battleship Yamato 2199 (2012) episode 18 (720p 8 bit AAC)[BA70BA9C]"
            ),
            "[Nubles] Space Battleship Yamato 2199 (2012) episode 18 (720p 8 bit AAC)[BA70BA9C]"
        );
        assert_eq!(
            remove_extension(
                "[Nubles] Space Battleship Yamato 2199 (2012) episode 18 (720p 10 bit AAC)[1F56D642]"
            ),
            "[Nubles] Space Battleship Yamato 2199 (2012) episode 18 (720p 10 bit AAC)[1F56D642]"
        );
        assert_eq!(
            remove_extension("【MMZYSUB】★【Golden Time】[24（END）][GB][720P_MP4]"),
            "【MMZYSUB】★【Golden Time】[24（END）][GB][720P_MP4]"
        );
        assert_eq!(remove_extension("Vol.01"), "Vol.01");
        assert_eq!(
            remove_extension(
                "Attack on Titan - Episode 3 - A Dim Light Amid Despair / Humanity's Comeback, Part 1"
            ),
            "Attack on Titan - Episode 3 - A Dim Light Amid Despair / Humanity's Comeback, Part 1"
        );
        assert_eq!(
            remove_extension("DRAMAtical Murder Episode 1 - Data_01_Login"),
            "DRAMAtical Murder Episode 1 - Data_01_Login"
        );
        assert_eq!(
            remove_extension("[Coalgirls]_White_Album_1-13_(1280×720_Blu-Ray_FLAC)"),
            "[Coalgirls]_White_Album_1-13_(1280×720_Blu-Ray_FLAC)"
        );
        assert_eq!(
            remove_extension(
                "[CH] Sword Art Online Extra Edition Dual Audio [BD 480p][10bitH.264+Vorbis]"
            ),
            "[CH] Sword Art Online Extra Edition Dual Audio [BD 480p][10bitH.264+Vorbis]"
        );
        assert_eq!(
            remove_extension(
                "Episode 14 Ore no Imouto ga Konnani Kawaii Wake ga Nai. (saison 2) VOSTFR"
            ),
            "Episode 14 Ore no Imouto ga Konnani Kawaii Wake ga Nai. (saison 2) VOSTFR"
        );
        assert_eq!(
            remove_extension("Ep. 01 - The Boy in the Iceberg"),
            "Ep. 01 - The Boy in the Iceberg"
        );
        assert_eq!(
            remove_extension(
                "Byousoku 5 Centimeter [Blu-Ray][1920x1080 H.264][2.0ch AAC][SOFTSUBS]"
            ),
            "Byousoku 5 Centimeter [Blu-Ray][1920x1080 H.264][2.0ch AAC][SOFTSUBS]"
        );
        assert_eq!(
            remove_extension("[LRL] 1001 Nights (1998) [DVD]"),
            "[LRL] 1001 Nights (1998) [DVD]"
        );
        assert_eq!(remove_extension("[Anime"), "[Anime");
        assert_eq!(
            remove_extension("Gekkan Shoujo Nozaki-kun [HorribleSubs] (1080p)"),
            "Gekkan Shoujo Nozaki-kun [HorribleSubs] (1080p)"
        );
        assert_eq!(
            remove_extension("[BM&T] Toradora! - 07v2 - Pool Opening [720p Hi10 ] [BD] [8F59F2BA]"),
            "[BM&T] Toradora! - 07v2 - Pool Opening [720p Hi10 ] [BD] [8F59F2BA]"
        );
        assert_eq!(
            remove_extension(
                "[EveTaku] AKB0048 Vol.03 - Making of Kibou-ni-Tsuite Music Video (BDRip 1080i H.264-Hi10P FLAC)[C09462E2]"
            ),
            "[EveTaku] AKB0048 Vol.03 - Making of Kibou-ni-Tsuite Music Video (BDRip 1080i H.264-Hi10P FLAC)[C09462E2]"
        );
        assert_eq!(
            remove_extension("[DmonHiro] Magi - The Labyrinth Of Magic - Vol.1v2 (BD, 720p)"),
            "[DmonHiro] Magi - The Labyrinth Of Magic - Vol.1v2 (BD, 720p)"
        );
        assert_eq!(
            remove_extension(
                "[tlacatlc6] Natsume Yuujinchou Shi Vol. 1v2 & Vol. 2 (BD 1280x720 x264 AAC)"
            ),
            "[tlacatlc6] Natsume Yuujinchou Shi Vol. 1v2 & Vol. 2 (BD 1280x720 x264 AAC)"
        );
        assert_eq!(
            remove_extension("[Tsundere] Hyouka - 01v2-04 [BDRip h264 1920x1080 10bit FLAC]"),
            "[Tsundere] Hyouka - 01v2-04 [BDRip h264 1920x1080 10bit FLAC]"
        );
        assert_eq!(
            remove_extension(
                "[Doki] Nogizaka Haruka no Himitsu - Purezza - 01v2-03v2 (1280x720 h264 AAC)"
            ),
            "[Doki] Nogizaka Haruka no Himitsu - Purezza - 01v2-03v2 (1280x720 h264 AAC)"
        );
        assert_eq!(
            remove_extension(
                "Fairy Tail - S06E32 - Tartaros Arc Iron Fist of the Fire Dragon [Episode 83]"
            ),
            "Fairy Tail - S06E32 - Tartaros Arc Iron Fist of the Fire Dragon [Episode 83]"
        );
        assert_eq!(
            remove_extension("Noragami - S02E06 - What Must Be Done [Episode 6]"),
            "Noragami - S02E06 - What Must Be Done [Episode 6]"
        );
        assert_eq!(
            remove_extension("[Harunatsu] Classroom Crisis - Vol.1 [BD 720p-AAC]"),
            "[Harunatsu] Classroom Crisis - Vol.1 [BD 720p-AAC]"
        );
        assert_eq!(
            remove_extension("[GS] Classroom Crisis Vol.1&2 (BD 1080p 10bit FLAC)"),
            "[GS] Classroom Crisis Vol.1&2 (BD 1080p 10bit FLAC)"
        );
        assert_eq!(
            remove_extension("[Infantjedi] Norn9 - Norn + Nonetto - 12"),
            "[Infantjedi] Norn9 - Norn + Nonetto - 12"
        );
        assert_eq!(
            remove_extension("Dragon_Ball_Z_Movies_8_&_10_[720p,BluRay,DTS,x264]_-_THORA"),
            "Dragon_Ball_Z_Movies_8_&_10_[720p,BluRay,DTS,x264]_-_THORA"
        );
        assert_eq!(
            remove_extension("[HorribleSubs] Momokuri - 01+02 [720p]"),
            "[HorribleSubs] Momokuri - 01+02 [720p]"
        );
        assert_eq!(remove_extension(""), "");
    }

    #[test]
    fn other_extension() {
        assert_eq!(
            remove_extension(
                "[Yuurisan-Subs]_Darker_than_Black_-_Gemini_of_the_Meteor_-_01v2_[65274FDE].patch.7z"
            ),
            "[Yuurisan-Subs]_Darker_than_Black_-_Gemini_of_the_Meteor_-_01v2_[65274FDE].patch"
        );
        assert_eq!(
            remove_extension("[Hard-Boiled FS]FullMetalAlchemist_09.rmvb"),
            "[Hard-Boiled FS]FullMetalAlchemist_09"
        );
    }

    #[test]
    fn try_bypass() {
        assert_eq!(
            remove_extension(
                "[Yuurisan-Subs]_Darker_than_Black_-_Gemini_of_the_Meteor_-_01v2_[65274FDE].7z.patch.7z"
            ),
            "[Yuurisan-Subs]_Darker_than_Black_-_Gemini_of_the_Meteor_-_01v2_[65274FDE].7z.patch"
        );
    }
}
