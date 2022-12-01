program aoc22d11;
{$mode objfpc}{$H+}

uses strutils;

function readInput(): string;
var
  line: string;
  wasLastEmpty: boolean;
begin
  Result := '';
  wasLastEmpty := false;
  while True do
  begin
    line := '';
    readln(line);
    if line = '' then
    begin
      if wasLastEmpty then
        break
      else 
        wasLastEmpty := true;
    end else
      wasLastEmpty := false;
    if Result = '' then
      Result := line
    else
      Result := Result + #10 + line;
  end;
end;

var
  i: integer;
  calories: integer;
  currentElf: integer;
  totalInput: string;
  line: string;
  lines: array of string;
  elves: array of integer;
  c: integer;
  biggestNumber: integer;
begin
  totalInput := readInput;
  lines := SplitString(totalInput, #10);
  setlength(elves, 1);
  currentElf := 0;
  for i := 0 to length(lines) - 1 do
  begin
    line := lines[i];
    if line = '' then
    begin
      if elves[currentElf] = 0 then
        break;
      setLength(elves, length(elves)+1);
      currentElf := currentElf + 1;
    end else begin
      val(lines[i], calories, c);
      if c = 0 then
        elves[currentElf] := elves[currentElf] + calories;
    end;
  end;
  
  biggestNumber := 0;
  
  for i := 0 to length(elves) - 1 do
  begin
    if elves[i] > biggestNumber then
      biggestNumber := elves[i];
  end;
  
  writeln(biggestNumber);
end.
  
