unit WlxPlugin;

{ Copyright (C) 2026 Martin Brozkeff Malec
  Licensed under the EUPL, Version 1.2.

  Minimal declarations from the Total Commander-compatible WLX interface,
  sufficient for this standalone plugin. }

{$mode objfpc}{$H+}

interface

type
  THandle = {$IFDEF CPU64}QWord{$ELSE}LongWord{$ENDIF};
  HWND = type THandle;

const
  wlxInvalidHandle: THandle = THandle(0);

implementation

end.
