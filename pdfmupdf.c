/* Dynamic MuPDF adapter for the Double Commander PDF WLX plugin. */
#include <dlfcn.h>
#include <stddef.h>
#include <mupdf/fitz.h>

typedef fz_context *(*new_context_fn)(const fz_alloc_context *, const fz_locks_context *, size_t, const char *);
typedef void (*drop_context_fn)(fz_context *);
typedef void (*register_handlers_fn)(fz_context *);
typedef fz_document *(*open_document_fn)(fz_context *, const char *);
typedef int (*count_pages_fn)(fz_context *, fz_document *);
typedef void (*drop_document_fn)(fz_context *, fz_document *);
typedef fz_page *(*load_page_fn)(fz_context *, fz_document *, int);
typedef void (*drop_page_fn)(fz_context *, fz_page *);
typedef fz_colorspace *(*device_rgb_fn)(fz_context *);
typedef fz_pixmap *(*new_pixmap_fn)(fz_context *, fz_page *, fz_matrix, fz_colorspace *, int);
typedef void (*drop_pixmap_fn)(fz_context *, fz_pixmap *);
typedef void (*save_png_fn)(fz_context *, fz_pixmap *, const char *);
typedef fz_jmp_buf *(*push_try_fn)(fz_context *);
typedef int (*try_state_fn)(fz_context *);

static void *mupdf_handle;
static int mupdf_loaded;
static new_context_fn p_new_context;
static drop_context_fn p_drop_context;
static register_handlers_fn p_register_handlers;
static open_document_fn p_open_document;
static count_pages_fn p_count_pages;
static drop_document_fn p_drop_document;
static load_page_fn p_load_page;
static drop_page_fn p_drop_page;
static device_rgb_fn p_device_rgb;
static new_pixmap_fn p_new_pixmap;
static drop_pixmap_fn p_drop_pixmap;
static save_png_fn p_save_png;
static push_try_fn p_push_try;
static try_state_fn p_do_try;
static try_state_fn p_do_always;
static try_state_fn p_do_catch;

#define fz_push_try p_push_try
#define fz_do_try p_do_try
#define fz_do_always p_do_always
#define fz_do_catch p_do_catch
#define LOAD_API(variable, symbol) do { *(void **)(&variable) = dlsym(mupdf_handle, "fz_" symbol); if (!variable) return 0; } while (0)

static int load_mupdf(void)
{
    static const char *names[] = {"libmupdf.so.25", "libmupdf.so.1", "libmupdf.so", NULL};
    size_t i;
    if (mupdf_loaded) return 1;
    for (i = 0; names[i] != NULL && mupdf_handle == NULL; ++i)
        mupdf_handle = dlopen(names[i], RTLD_NOW | RTLD_LOCAL);
    if (!mupdf_handle) return 0;
    LOAD_API(p_new_context, "new_context_imp");
    LOAD_API(p_drop_context, "drop_context");
    LOAD_API(p_register_handlers, "register_document_handlers");
    LOAD_API(p_open_document, "open_document");
    LOAD_API(p_count_pages, "count_pages");
    LOAD_API(p_drop_document, "drop_document");
    LOAD_API(p_load_page, "load_page");
    LOAD_API(p_drop_page, "drop_page");
    LOAD_API(p_device_rgb, "device_rgb");
    LOAD_API(p_new_pixmap, "new_pixmap_from_page");
    LOAD_API(p_drop_pixmap, "drop_pixmap");
    LOAD_API(p_save_png, "save_pixmap_as_png");
    LOAD_API(p_push_try, "push_try");
    LOAD_API(p_do_try, "do_try");
    LOAD_API(p_do_always, "do_always");
    LOAD_API(p_do_catch, "do_catch");
    mupdf_loaded = 1;
    return 1;
}

int pdf_mupdf_page_count(const char *filename)
{
    fz_context *ctx = NULL;
    fz_document *doc = NULL;
    int result = -1;
    if (!load_mupdf()) return -1;
    ctx = p_new_context(NULL, NULL, FZ_STORE_DEFAULT, FZ_VERSION);
    if (!ctx) return -1;
    p_register_handlers(ctx);
    fz_try(ctx) {
        doc = p_open_document(ctx, filename);
        result = p_count_pages(ctx, doc);
    }
    fz_always(ctx) { p_drop_document(ctx, doc); }
    fz_catch(ctx) { result = -1; }
    p_drop_context(ctx);
    return result;
}

int pdf_mupdf_render_page(const char *filename, int page_number, int dpi,
                          const char *png_filename)
{
    fz_context *ctx = NULL;
    fz_document *doc = NULL;
    fz_page *page = NULL;
    fz_pixmap *pix = NULL;
    fz_matrix transform = {0};
    int result = -1;
    if (!load_mupdf()) return -1;
    ctx = p_new_context(NULL, NULL, FZ_STORE_DEFAULT, FZ_VERSION);
    if (!ctx) return -1;
    p_register_handlers(ctx);
    transform.a = transform.d = dpi / 72.0f;
    fz_try(ctx) {
        doc = p_open_document(ctx, filename);
        page = p_load_page(ctx, doc, page_number);
        pix = p_new_pixmap(ctx, page, transform, p_device_rgb(ctx), 0);
        p_save_png(ctx, pix, png_filename);
        result = 0;
    }
    fz_always(ctx) {
        p_drop_pixmap(ctx, pix);
        p_drop_page(ctx, page);
        p_drop_document(ctx, doc);
    }
    fz_catch(ctx) { result = -1; }
    p_drop_context(ctx);
    return result;
}
