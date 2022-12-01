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
  biggestNumbers: array[0..2] of integer;
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
  
  biggestNumbers[0] := 0;
  biggestNumbers[1] := 0;
  biggestNumbers[2] := 0;
  
  for i := 0 to length(elves) - 1 do
  begin
    if elves[i] > biggestNumbers[0] then
    begin
      biggestNumbers[2] := biggestNumbers[1];
      biggestNumbers[1] := biggestNumbers[0];
      biggestNumbers[0] := elves[i];
    end else if elves[i] > biggestNumbers[1] then
    begin
      biggestNumbers[2] := biggestNumbers[1];
      biggestNumbers[1] := elves[i];    
    end else if elves[i] > biggestNumbers[2] then
    begin
      biggestNumbers[2] := elves[i];
    end;
  end;
  
  writeln(biggestNumbers[0] + biggestNumbers[1] + biggestNumbers[2]);
end.
  
