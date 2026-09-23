// Copyright (C) 2026 Martin Brozkeff Malec
// SPDX-License-Identifier: EUPL-1.2 OR GPL-2.0-or-later OR AGPL-3.0-or-later

#include <poppler-qt5.h>

#include <QByteArray>
#include <QFile>
#include <QImage>
#include <QPixmap>
#include <QString>

#include <cstddef>
#include <memory>

#include "qt5_shim.hpp"

extern "C" void *pdf_wlx_poppler_create(void *parent_handle,
                                         const char *file_name,
                                         unsigned int render_dpi) noexcept {
  if (parent_handle == nullptr || file_name == nullptr || render_dpi == 0) {
    return nullptr;
  }

  try {
    const QString path = QFile::decodeName(QByteArray(file_name));
    std::unique_ptr<Poppler::Document> document(Poppler::Document::load(path));
    if (!document || document->isLocked() || document->isEncrypted() ||
        document->numPages() <= 0) {
      return nullptr;
    }
    const int document_page_count = document->numPages();

    const auto backends = Poppler::Document::availableRenderBackends();
    if (!backends.contains(Poppler::Document::SplashBackend)) {
      return nullptr;
    }
    document->setRenderBackend(Poppler::Document::SplashBackend);
    document->setRenderHint(Poppler::Document::Antialiasing);
    document->setRenderHint(Poppler::Document::TextAntialiasing);
    document->setRenderHint(Poppler::Document::TextHinting);
    document->setRenderHint(Poppler::Document::TextSlightHinting);

    std::unique_ptr<Poppler::Page> page(document->page(0));
    if (!page) {
      return nullptr;
    }
    const QImage image = page->renderToImage(render_dpi, render_dpi);
    if (image.isNull()) {
      return nullptr;
    }
    const QPixmap pixmap = QPixmap::fromImage(image);

    return pdf_wlx_qt5_create_from_pixmaps(
        parent_handle, &pixmap, 1, static_cast<std::size_t>(document_page_count));
  } catch (...) {
    return nullptr;
  }
}
