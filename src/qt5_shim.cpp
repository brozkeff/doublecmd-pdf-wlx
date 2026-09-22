// Copyright (C) 2026 Martin Brozkeff Malec
// Licensed under the EUPL, Version 1.2. This is the complete Qt unsafe shim.

#include <QLabel>
#include <QKeySequence>
#include <QPixmap>
#include <QShortcut>
#include <QScrollArea>
#include <QString>
#include <QTimer>
#include <QVBoxLayout>
#include <QWidget>

#include <cstddef>
#include <memory>
#include <new>

extern "C" void *pdf_wlx_qt5_create(void *parent_handle,
                                     const char *const *page_paths,
                                     std::size_t page_count,
                                     std::size_t document_page_count) noexcept {
  if (parent_handle == nullptr || page_paths == nullptr || page_count == 0) {
    return nullptr;
  }

  try {
    auto *parent = static_cast<QWidget *>(parent_handle);
    auto scroll = std::make_unique<QScrollArea>(parent);
    auto *escape_shortcut = new QShortcut(QKeySequence(Qt::Key_Escape), scroll.get());
    escape_shortcut->setContext(Qt::WidgetWithChildrenShortcut);
    auto *viewer_window = scroll->window();
    QObject::connect(escape_shortcut, &QShortcut::activated, scroll.get(),
                     [viewer_window]() {
                       if (viewer_window != nullptr) {
                         // Defer host teardown until the key event has returned.
                         QTimer::singleShot(0, viewer_window, &QWidget::close);
                       }
                     });

    auto *content = new QWidget(scroll.get());
    auto *page_layout = new QVBoxLayout(content);
    page_layout->setContentsMargins(0, 0, 0, 0);
    page_layout->setSpacing(8);

    if (document_page_count > page_count) {
      auto *indicator = new QLabel(
          QString("Page 1 of %1").arg(static_cast<qulonglong>(document_page_count)),
          content);
      indicator->setAlignment(Qt::AlignHCenter | Qt::AlignVCenter);
      page_layout->addWidget(indicator);
    }

    for (std::size_t index = 0; index < page_count; ++index) {
      if (page_paths[index] == nullptr) {
        return nullptr;
      }
      QPixmap pixmap(QString::fromUtf8(page_paths[index]));
      if (pixmap.isNull()) {
        return nullptr;
      }
      auto *label = new QLabel(content);
      label->setAlignment(Qt::AlignHCenter | Qt::AlignTop);
      label->setPixmap(pixmap);
      page_layout->addWidget(label);
    }

    page_layout->addStretch();
    scroll->setWidget(content);
    scroll->setWidgetResizable(true);
    scroll->show();
    return scroll.release();
  } catch (...) {
    return nullptr;
  }
}

extern "C" void pdf_wlx_qt5_destroy(void *window_handle) noexcept {
  try {
    delete static_cast<QWidget *>(window_handle);
  } catch (...) {
    // No C++ exception may cross the C ABI boundary.
  }
}
