use oxidite::prelude::*;
use oxidite::template::{Context, TemplateEngine};
use serde::Serialize;

pub fn register(router: &mut Router) {
    router.get("/", index);
    router.get("/chapter/:slug", chapter);
    router.get("/sitemap.xml", sitemap);
}

async fn sitemap(_req: Request) -> Result<Response> {
    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
    <url>
        <loc>https://thelongmiddle.g24sec.com/</loc>
        <changefreq>weekly</changefreq>
        <priority>1.0</priority>
    </url>"#);

    for (slug, _) in CHAPTERS {
        xml.push_str(&format!(
            r#"
    <url>
        <loc>https://thelongmiddle.g24sec.com/chapter/{}</loc>
        <changefreq>monthly</changefreq>
        <priority>0.8</priority>
    </url>"#,
            slug
        ));
    }

    xml.push_str("\n</urlset>");

    Ok(Response::new(oxidite::http::StatusCode::OK)
        .header("Content-Type", "application/xml")
        .body(xml))
}

#[derive(Clone, Serialize)]
struct Chapter {
    number: u8,
    slug: String,
    title: String,
}

const CHAPTERS: &[(&str, &str)] = &[
    ("the-fracture", "The Fracture"),
    ("the-kid-in-the-video", "The Kid in the Video"),
    ("the-collapse-and-the-rise", "The Collapse and the Rise"),
    ("the-script-kiddie-era", "The Script Kiddie Era"),
    ("the-invisible-engineer", "The Invisible Engineer"),
    ("the-systems-no-one-saw-coming", "The Systems No One Saw Coming"),
    ("jarvis-and-the-silence", "Jarvis and the Silence"),
    ("the-maturation", "The Maturation"),
    ("the-cost-of-invisible-labor", "The Cost of Invisible Labor"),
    ("the-rejections", "The Rejections"),
    ("the-break", "The Break"),
    ("the-architect-in-the-room", "The Architect in the Room"),
    ("the-long-middle", "The Long Middle"),
    ("the-open-question", "The Open Question"),
    ("a-note-on-what-comes-next", "A Note on What Comes Next"),
];

fn build_chapters() -> Vec<Chapter> {
    CHAPTERS
        .iter()
        .enumerate()
        .map(|(i, (slug, title))| Chapter {
            number: (i + 1) as u8,
            slug: slug.to_string(),
            title: title.to_string(),
        })
        .collect()
}

fn load_engine() -> Result<TemplateEngine> {
    let mut engine = TemplateEngine::new();
    engine
        .load_dir("templates")
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    Ok(engine)
}

fn set_chapters(ctx: &mut Context) {
    ctx.set("chapters", build_chapters());
}

async fn index(_req: Request) -> Result<Response> {
    let engine = load_engine()?;
    let mut ctx = Context::new();

    ctx.set("page_title", "The Long Middle: Built First, Seen Last | Meshack Bahati");
    ctx.set(
        "page_description",
        "A first-person memoir about building AI systems, cybersecurity platforms, and Rust frameworks years ahead of their time while remaining invisible in the global tech industry.",
    );
    ctx.set("canonical_url", "/");
    set_chapters(&mut ctx);

    let body = engine
        .render("index.html", &ctx)
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    Ok(Response::html(body))
}

async fn chapter(req: Request) -> Result<Response> {
    let slug = req
        .uri()
        .path()
        .strip_prefix("/chapter/")
        .unwrap_or("")
        .to_string();

    let chapters = build_chapters();
    let idx = chapters
        .iter()
        .position(|c| c.slug == slug)
        .ok_or_else(|| Error::NotFound(format!("Chapter not found: {}", slug)))?;

    let current = &chapters[idx];
    let engine = load_engine()?;
    let mut ctx = Context::new();

    ctx.set(
        "page_title",
        format!("{} | The Long Middle", current.title),
    );
    ctx.set(
        "page_description",
        format!(
            "Part {} of The Long Middle: Built First, Seen Last by Meshack Bahati.",
            current.number
        ),
    );
    ctx.set("canonical_url", format!("/chapter/{}", slug));
    ctx.set("current_chapter", &slug);
    ctx.set("chapter_number", current.number);
    ctx.set("chapter_title", &current.title);
    set_chapters(&mut ctx);

    // Set the flag for the current chapter include
    let flag = format!("is_chapter_{:02}", idx + 1);
    ctx.set(&flag, true);

    // Prev / next
    if idx > 0 {
        ctx.set("prev_slug", &chapters[idx - 1].slug);
    }
    if idx + 1 < chapters.len() {
        ctx.set("next_slug", &chapters[idx + 1].slug);
    }

    let body = engine
        .render("chapter.html", &ctx)
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    Ok(Response::html(body))
}
