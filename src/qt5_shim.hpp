// Copyright (C) 2026 Martin Brozkeff Malec
// SPDX-License-Identifier: EUPL-1.2 OR GPL-2.0-or-later OR AGPL-3.0-or-later

#pragma once

#include <QPixmap>

#include <cstddef>

void *pdf_wlx_qt5_create_from_pixmaps(void *parent_handle, const QPixmap *pixmaps,
                                      std::size_t image_count,
                                      std::size_t document_page_count) noexcept;
