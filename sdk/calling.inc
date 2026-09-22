{ Copyright (C) 2026 Martin Brozkeff Malec
  Licensed under the EUPL, Version 1.2. }

{$MACRO ON}
{$IFDEF MSWINDOWS}
{$DEFINE dcpcall:=stdcall}
{$ELSE}
{$DEFINE dcpcall:=cdecl}
{$ENDIF}
