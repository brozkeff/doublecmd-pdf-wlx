// Copyright (C) 2026 Martin Brozkeff Malec
// Licensed under the EUPL, Version 1.2. This is the complete Qt unsafe shim.

#include <QLabel>
#include <QPixmap>
#include <QScrollArea>
#include <QString>
#include <QVBoxLayout>
#include <QWidget>

#include <cstddef>
#include <memory>
#include <new>

extern "C" void *pdf_wlx_qt5_create(void *parent_handle,
                                     const char *const *page_paths,
                                     std::size_t page_count) noexcept {
  if (parent_handle == nullptr || page_paths == nullptr || page_count == 0) {
    return nullptr;
  }

  try {
    auto *parent = static_cast<QWidget *>(parent_handle);
    auto scroll = std::make_unique<QScrollArea>(parent);
    auto *content = new QWidget(scroll.get());
    auto *layout = new QVBoxLayout(content);
    layout->setContentsMargins(0, 0, 0, 0);
    layout->setSpacing(8);

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
      layout->addWidget(label);
    }

    layout->addStretch();
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
