library PdfWlx;

{$mode objfpc}{$H+}
{$include sdk/calling.inc}

uses
  {$IFDEF UNIX}cmem, cthreads,{$ENDIF}
  Classes, SysUtils, Process, LazUTF8, WlxPlugin,
  {$IF DEFINED(LCLGTK2)}gtk2{$ELSEIF DEFINED(LCLGTK3)}LazGtk3{$ELSEIF DEFINED(LCLQT5)}qt5, qtwidgets{$ELSEIF DEFINED(LCLQT6)}qt6, qtwidgets{$ENDIF};

const
  RenderDpi = 120;
  MaxPages = 128;

function MuPdfPageCount(FileName: PAnsiChar): Integer; cdecl; external name 'pdf_mupdf_page_count';
function MuPdfRenderPage(FileName: PAnsiChar; PageNumber, Dpi: Integer; PngFile: PAnsiChar): Integer; cdecl; external name 'pdf_mupdf_render_page';

function TempPrefix: String;
begin
  Result := IncludeTrailingPathDelimiter(GetTempDir(False)) +
    'doublecmd-pdf-' + IntToStr(GetProcessID) + '-';
end;

procedure DeletePages(Pages: TStringList); forward;

function RenderPagesWithMutool(const FileName: String; Pages: TStringList): Boolean;
var
  Runner: TProcess;
  Prefix, Path: String;
  I: Integer;
begin
  Result := False;
  Pages.Clear;
  Prefix := TempPrefix;
  Runner := TProcess.Create(nil);
  try
    Runner.Executable := 'mutool';
    Runner.Parameters.Add('draw');
    Runner.Parameters.Add('-q');
    Runner.Parameters.Add('-r');
    Runner.Parameters.Add(IntToStr(RenderDpi));
    Runner.Parameters.Add('-o');
    Runner.Parameters.Add(Prefix + '%d.png');
    Runner.Parameters.Add(FileName);
    Runner.Parameters.Add('1-' + IntToStr(MaxPages));
    Runner.Options := [poWaitOnExit];
    Runner.Execute;
    if Runner.ExitStatus <> 0 then Exit;
    for I := 1 to MaxPages do
    begin
      Path := Prefix + IntToStr(I) + '.png';
      if not FileExists(Path) then Break;
      Pages.Add(Path);
    end;
    Result := Pages.Count > 0;
  finally
    Runner.Free;
  end;
end;

function RenderPages(const FileName: String; Pages: TStringList): Boolean;
var
  Count, I: Integer;
  Path: String;
begin
  Result := False;
  Count := MuPdfPageCount(PAnsiChar(UTF8Encode(FileName)));
  if (Count >= 1) and (Count <= MaxPages) then
  begin
    for I := 0 to Count - 1 do
    begin
      Path := TempPrefix + IntToStr(I + 1) + '.png';
      if MuPdfRenderPage(PAnsiChar(UTF8Encode(FileName)), I, RenderDpi,
        PAnsiChar(UTF8Encode(Path))) <> 0 then Break;
      Pages.Add(Path);
    end;
    if Pages.Count = Count then
    begin
      Result := True;
      Exit;
    end;
  end;
  DeletePages(Pages);
  Result := RenderPagesWithMutool(FileName, Pages);
end;

procedure DeletePages(Pages: TStringList);
var I: Integer;
begin
  for I := 0 to Pages.Count - 1 do DeleteFile(Pages[I]);
end;

{$IFDEF LCLGTK2}
function ListLoad(ParentWin: HWND; FileToLoad: PAnsiChar; ShowFlags: Integer): HWND; dcpcall;
var Scroll: PGtkScrolledWindow; Box, Image: PGtkWidget; Pages: TStringList; I: Integer; P: PAnsiChar;
begin
  Pages := TStringList.Create;
  try
    if (FileToLoad = nil) or not RenderPages(String(FileToLoad), Pages) then Exit(wlxInvalidHandle);
    Scroll := PGtkScrolledWindow(gtk_scrolled_window_new(nil, nil));
    Box := gtk_vbox_new(False, 8);
    for I := 0 to Pages.Count - 1 do begin P := PAnsiChar(Pages[I]); Image := PGtkWidget(gtk_image_new_from_file(P)); gtk_box_pack_start(PGtkBox(Box), Image, False, False, 0); end;
    gtk_container_add(PGtkContainer(Scroll), Box); gtk_container_add(PGtkContainer(ParentWin), PGtkWidget(Scroll)); gtk_widget_show_all(PGtkWidget(Scroll));
    Result := HWND(Scroll);
  except Result := wlxInvalidHandle; end;
  DeletePages(Pages); Pages.Free;
end;
procedure ListCloseWindow(ListWin: HWND); dcpcall; begin gtk_widget_destroy(PGtkWidget(ListWin)); end;
{$ELSE}
{$IFDEF LCLGTK3}
function ListLoad(ParentWin: HWND; FileToLoad: PAnsiChar; ShowFlags: Integer): HWND; dcpcall;
var Scroll: PGtkScrolledWindow; Box, Image: PGtkWidget; Pages: TStringList; I: Integer; P: PAnsiChar;
begin
  Pages := TStringList.Create;
  try
    if (FileToLoad = nil) or not RenderPages(String(FileToLoad), Pages) then Exit(wlxInvalidHandle);
    Scroll := PGtkScrolledWindow(gtk_scrolled_window_new(nil, nil)); Box := PGtkWidget(gtk_box_new(GTK_ORIENTATION_VERTICAL, 8));
    for I := 0 to Pages.Count - 1 do begin P := PAnsiChar(Pages[I]); Image := PGtkWidget(gtk_image_new_from_file(P)); gtk_box_pack_start(PGtkBox(Box), Image, False, False, 0); end;
    gtk_container_add(PGtkContainer(Scroll), Box); gtk_container_add(PGtkContainer(ParentWin), PGtkWidget(Scroll)); gtk_widget_show_all(PGtkWidget(Scroll)); Result := HWND(Scroll);
  except Result := wlxInvalidHandle; end;
  DeletePages(Pages); Pages.Free;
end;
procedure ListCloseWindow(ListWin: HWND); dcpcall; begin gtk_widget_destroy(PGtkWidget(ListWin)); end;
{$ELSE}
function ListLoad(ParentWin: HWND; FileToLoad: PAnsiChar; ShowFlags: Integer): HWND; dcpcall;
var Container, LabelWidget: QWidgetH; Layout: QLayoutH; Pixmap: QPixmapH; Pages: TStringList; I: Integer;
begin
  Pages := TStringList.Create;
  try
    if (FileToLoad = nil) or not RenderPages(String(FileToLoad), Pages) then Exit(wlxInvalidHandle);
    Container := QWidget_create(QWidgetH(ParentWin)); Layout := QVBoxLayout_create(Container);
    for I := 0 to Pages.Count - 1 do begin Pixmap := QPixmap_create(PAnsiChar(UTF8Encode(Pages[I]))); LabelWidget := QLabel_create(Container); QLabel_setPixmap(QLabelH(LabelWidget), Pixmap); QLayout_addWidget(Layout, LabelWidget); QPixmap_destroy(Pixmap); end;
    QWidget_show(Container); Result := HWND(Container);
  except Result := wlxInvalidHandle; end;
  DeletePages(Pages); Pages.Free;
end;
procedure ListCloseWindow(ListWin: HWND); dcpcall; begin QWidget_destroy(QWidgetH(ListWin)); end;
{$ENDIF}
{$ENDIF}

procedure ListGetDetectString(DetectString: PAnsiChar; MaxLen: Integer); dcpcall;
begin StrPLCopy(DetectString, 'EXT="PDF"', MaxLen - 1); end;
function PdfWlxLicense: PAnsiChar; dcpcall; begin Result := 'Copyright (C) 2026 Martin Brozkeff Malec; licensed under the EUPL 1.2'; end;
function PdfWlxVersion: PAnsiChar; dcpcall; begin Result := '0.1.0'; end;

exports ListLoad, ListCloseWindow, ListGetDetectString, PdfWlxLicense, PdfWlxVersion;
end.
