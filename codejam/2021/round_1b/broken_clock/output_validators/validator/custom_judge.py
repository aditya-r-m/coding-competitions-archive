# Copyright 2011 Google Inc. All Rights Reserved.
"""Basic utilities for custom judges."""

import logging


# Copied from judge.py
def _utils_Tokenize(text, case_sensitive=True):
  """Converts a block of text into a two-dimensional array of strings.

  Args:
    text: A block of text, probably either a contestant or generator output.
    case_sensitive: Whether all text should be converted to lower-case.

  Returns:
    A two-dimensional array of strings. There is one element for each non-blank
    row in the output, and there is one inner element for each token on that
    row. If text contains any characters outside ASCII range 32-126 (with the
    exception of tabs, carriage returns, and line feeds), None is returned.
  """
  if not case_sensitive:
    text = text.lower()
  text = text.replace('\t', ' ').replace('\r', '\n')
  for char in text:
    if not (32 <= ord(char) <= 126) and char != '\n':
      return None
  return [
      filter(None, row.split(' ')) for row in text.split('\n') if row.strip()
  ]


def _utils_TokenizeAndSplitCases(output_file,
                                 attempt,
                                 num_cases,
                                 case_sensitive=False):
  """Tokenizes the generator output file and attempt file by case number.

  This is similar to Tokenize except that:
    - It applies to both output_file and attempt.
    - The results are 3-dimensional vectors split by case number, and with the
      "Case #N:" tokens removed.
    - There could be empty rows due to the previous.
    - The number of cases in the output file and attempt must match num_cases.
    - An error string is returned if something is incorrect.

  Args:
    output_file: The output file, as given in FindError.
    attempt: The attempt file, as given in FindError.
    num_cases: The number of cases in the input file.
    case_sensitive: Whether to run in case-sensitive mode (for everything except
      the word 'Case' itself).

  Returns:
    On success, tokenized_output, tokenized_attempt, None is returned. Each of
    these are 3-dimensional arrays of tokens, sorted by case number, line
    number, and token. On failure, None, None, error is returned.
  """

  def ProcessOneFile(text, num_cases):
    """Similar to TokenizeAndSplitCases except applied to only one file."""

    # Tokenize and validate ASCII-ness. Case insensitive checking allows, for
    # example, contestants to output "case #N:" instead of "Case #N:".

    tokenized_text = _utils_Tokenize(text, case_sensitive=case_sensitive)
    if tokenized_text is None:
      return None, 'Invalid or non-ASCII characters.'

    # Build our result in split text.
    split_text = []
    # Even if case-sensitivity is on, allow the contestant to use any form of
    # 'case', since some contestants may have gotten accustomed to the
    # older, more flexible rules.
    for line in tokenized_text:
      if (len(line) >= 2 and line[0].lower() == 'case' and
          line[1].startswith('#')):
        # This line is a "Case #N:" line.
        expected_case = 1 + len(split_text)
        if line[1] != ('#%d:' % expected_case):
          return None, ('Expected "case #%d:", found "%s %s".' %
                        (expected_case, line[0], line[1]))
        if expected_case > num_cases:
          return None, 'Too many cases.'
        split_text.append([line[2:]])
      else:
        # This line is any other kind of line.
        if not split_text:
          return None, 'File does not begin with "case #1:".'
        split_text[-1].append(line)

    # At the end, make sure we had enough cases.
    if len(split_text) < num_cases:
      return None, 'Too few cases.'
    return split_text, None

  # Parse the generator output file. If something is wrong here, log an error.
  split_output, error = ProcessOneFile(output_file, num_cases)
  if error:
    error = 'Invalid generator output file: %s' % error
    logging.error(error)
    return None, None, error

  # Parse the user output file attempt.
  split_attempt, error = ProcessOneFile(attempt, num_cases)
  if error:
    error = 'Invalid attempt file: %s' % error
    return None, None, error
  return split_output, split_attempt, None


def _utils_ToInteger(s, minimum_value=None, maximum_value=None):
  """Returns int(s) if s is an integer in the given range.

  Otherwise None.

  The range is inclusive. Also, leading zeroes and negative zeros are not
  allowed.

  Args:
    s: A string to convert to an integer.
    minimum_value: If not-None, then s must be at least this value.
    maximum_value: If not-None, then s must be at most this value.

  Returns:
    An integer in the given range or None.
  """
  try:
    value = int(s)
    if len(s) > 1 and s.startswith('0'):
      return None
    if s.startswith('-0'):
      return None
    if minimum_value is not None and value < minimum_value:
      return None
    if maximum_value is not None and value > maximum_value:
      return None
    return value
  except ValueError:
    return None


def _utils_ToFloat(s):
  """Returns float(s) if s is a float.

  Otherwise None.

  Disallows infinities and nans.

  Args:
    s: A string to convert to a float.

  Returns:
    An float or None.
  """
  try:
    x = float(s)
    if x not in [float('inf'), float('-inf')] and x == x:  # not NaN
      return x
    else:
      return None
  except ValueError:
    return None


"""Custom judge for Broken Clock, Code Jam 2021."""

import collections

_BAD_OUTPUT_ERROR_STR = 'Our output is incorrect:'
_BAD_OUTPUT_ERROR = (_BAD_OUTPUT_ERROR_STR + ' {}').format

_BAD_FORMAT_ERROR = 'Output is not well-formatted'
_WRONG_ANSWER = 'Time does not correspond to the angles'

_BLN = 10**9
_CIRCLE = 360 * 120 * _BLN
"""Parsed information for a single test case.

Attributes:
  a: angle of the first hand in the input.
  b: angle of the second hand in the input.
  c: angle of the third hand in the input.
"""
_CaseInput = collections.namedtuple('_CaseInput', ['a', 'b', 'c'])


def ParseInputFile(input_file):
  """Turns an input file into a list of _CaseInputs."""
  input_lines = _utils_Tokenize(input_file)[1:]  # skip the number of cases
  cases = []
  for input_line in input_lines:
    a, b, c = map(int, input_line)
    cases.append(_CaseInput(a, b, c))
  return cases


def VerifyOutput(lines, case):
  """Checks the output for a single test case.

  Args:
    lines: Tokenized lines (either from our I/OGen's output or a contestant's
      attempt.)
    case: A _CaseInput representing a single test case.

  Returns:
    None if the output is correct, or an error message.
  """

  def _angles(h, m, s, n):
    c = (_CIRCLE // 60 * s + _CIRCLE // 60 // _BLN * n) % _CIRCLE
    b = (_CIRCLE // 60 * m + _CIRCLE // 60 // 60 * s +
         _CIRCLE // 60 // 60 // _BLN * n) % _CIRCLE
    a = (_CIRCLE // 12 * h + _CIRCLE // 12 // 60 * m + _CIRCLE // 12 // 60 //
         60 * s + _CIRCLE // 12 // 60 // 60 // _BLN * n) % _CIRCLE
    return a, b, c

  def _normalize(a, b, c):
    result = None
    for a_, b_, c_ in [(a, b, c), (a, c, b), (b, a, c), (b, c, a), (c, a, b),
                       (c, b, a)]:
      x = (b_ - a_ + _CIRCLE) % _CIRCLE
      y = (c_ - b_ + _CIRCLE) % _CIRCLE
      if result is None or (x, y) < result:
        result = (x, y)
    return result

  if len(lines) != 1 or len(lines[0]) != 4:
    return _BAD_FORMAT_ERROR

  try:
    h, m, s, ns = map(int, lines[0])
  except:
    return _BAD_FORMAT_ERROR

  if h < 0 or h >= 12 or m < 0 or m >= 60 or s < 0 or s >= 60 or ns < 0 or ns >= 10**9:
    return _BAD_FORMAT_ERROR

  a, b, c = _angles(h, m, s, ns)
  if _normalize(case.a, case.b, case.c) != _normalize(a, b, c):
    return _WRONG_ANSWER

  return None


def VerifyCase(output_lines, attempt_lines, case):
  """Checks a single test case.

  Args:
    output_lines: Tokenized lines from our I/OGen's output.
    attempt_lines: Tokenized lines from the contestant's attempt.
    case: A CaseInput representing a single test case.

  Returns:
    An error, or None if there is no error.
  """

  output_error = VerifyOutput(output_lines, case)
  if output_error is not None:
    return _BAD_OUTPUT_ERROR(output_error)

  attempt_error = VerifyOutput(attempt_lines, case)
  if attempt_error is not None:
    return attempt_error

  return None


def FindError(unused_self, input_file, output_file, attempt_file):
  """See judge.Judge.FindError()."""
  input_cases = ParseInputFile(input_file)
  num_cases = len(input_cases)
  output_cases, attempt_cases, error = _utils_TokenizeAndSplitCases(
      output_file, attempt_file, num_cases, True)
  if error is not None:
    return error

  for case_num, (case, output_lines, attempt_lines) in enumerate(
      zip(input_cases, output_cases, attempt_cases), start=1):
    error = VerifyCase(output_lines, attempt_lines, case)
    if error is not None:
      je = 'JUDGE_ERROR ' if error.startswith(_BAD_OUTPUT_ERROR_STR) else ''
      return '{}Case #{}: {}'.format(je, case_num, error)

  # Everything passes.
  return None


class BrokenClockUnitTest():

  def assertIsNone(self, a):
    assert a is None

  def assertEqual(self, a, b):
    assert a == b, (a, b)

  input_file = """
4
43199000000000 43199000000000 43199000000000
0 1800000000000 21600000000000
1476000000001 2160000000001 3723000000001
1 12 720
""".lstrip()
  output_file = """
Case #1: 0 0 0 0
Case #2: 6 30 0 0
Case #3: 1 2 3 0
Case #4: 0 0 0 1
""".lstrip()

  def testCorrectAnswerDoesNotCauseError(self):
    self.assertIsNone(
        FindError(None, self.input_file, self.output_file, self.output_file))

  def testAlternativeCorrectAnswerDoesNotCauseError(self):
    self.assertIsNone(
        FindError(
            None, self.input_file, self.output_file, """
Case #1: 0 0 0 0
Case #2: 0 30 0 0
Case #3: 1 2 3 0
Case #4: 0 0 0 1
""".lstrip()))

  def testIncorrectAnswerCausesError(self):
    self.assertEqual(
        'Case #4: ' + _WRONG_ANSWER,
        FindError(
            None, self.input_file, self.output_file, """
Case #1: 0 0 0 0
Case #2: 0 30 0 0
Case #3: 1 2 3 0
Case #4: 0 0 1 0
""".lstrip()))

  def testEmptyAttemptCausesError(self):
    self.assertEqual('Invalid attempt file: Too few cases.',
                     FindError(None, self.input_file, self.output_file, ''))

  def testBadHeaderCausesError(self):
    self.assertEqual(
        'Invalid attempt file: Expected "case #1:", found "Case ##1:".',
        FindError(None, self.input_file, self.output_file,
                  'Case ##1: IMPOSSIBLE'))

  def testBadFormatCausesError(self):
    self.assertEqual(
        'Case #1: ' + _BAD_FORMAT_ERROR,
        FindError(
            None, self.input_file, self.output_file, """
Case #1:
Case #2: 0 30 0 0
Case #3: 1 2 3 0
Case #4: 0 0 1 0
""".lstrip()))

    self.assertEqual(
        'Case #1: ' + _BAD_FORMAT_ERROR,
        FindError(
            None, self.input_file, self.output_file, """
Case #1: 0 0 0
Case #2: 0 30 0 0
Case #3: 1 2 3 0
Case #4: 0 0 1 0
""".lstrip()))

    self.assertEqual(
        'Case #1: ' + _BAD_FORMAT_ERROR,
        FindError(
            None, self.input_file, self.output_file, """
Case #1: 0 0 0 0 0
Case #2: 0 30 0 0
Case #3: 1 2 3 0
Case #4: 0 0 1 0
""".lstrip()))

    self.assertEqual(
        'Case #1: ' + _BAD_FORMAT_ERROR,
        FindError(
            None, self.input_file, self.output_file, """
Case #1: 0 0.0 0 0
Case #2: 0 30 0 0
Case #3: 1 2 3 0
Case #4: 0 0 1 0
""".lstrip()))

    self.assertEqual(
        'Case #1: ' + _BAD_FORMAT_ERROR,
        FindError(
            None, self.input_file, self.output_file, """
Case #1: -1 0 0 0
Case #2: 0 30 0 0
Case #3: 1 2 3 0
Case #4: 0 0 1 0
""".lstrip()))

    self.assertEqual(
        'Case #1: ' + _BAD_FORMAT_ERROR,
        FindError(
            None, self.input_file, self.output_file, """
Case #1: 12 0 0 0
Case #2: 0 30 0 0
Case #3: 1 2 3 0
Case #4: 0 0 1 0
""".lstrip()))

    self.assertEqual(
        'Case #1: ' + _BAD_FORMAT_ERROR,
        FindError(
            None, self.input_file, self.output_file, """
Case #1: 0 -1 0 0
Case #2: 0 30 0 0
Case #3: 1 2 3 0
Case #4: 0 0 1 0
""".lstrip()))

    self.assertEqual(
        'Case #1: ' + _BAD_FORMAT_ERROR,
        FindError(
            None, self.input_file, self.output_file, """
Case #1: 0 60 0 0
Case #2: 0 30 0 0
Case #3: 1 2 3 0
Case #4: 0 0 1 0
""".lstrip()))

    self.assertEqual(
        'Case #1: ' + _BAD_FORMAT_ERROR,
        FindError(
            None, self.input_file, self.output_file, """
Case #1: 0 0 -1 0
Case #2: 0 30 0 0
Case #3: 1 2 3 0
Case #4: 0 0 1 0
""".lstrip()))

    self.assertEqual(
        'Case #1: ' + _BAD_FORMAT_ERROR,
        FindError(
            None, self.input_file, self.output_file, """
Case #1: 0 0 60 0
Case #2: 0 30 0 0
Case #3: 1 2 3 0
Case #4: 0 0 1 0
""".lstrip()))

    self.assertEqual(
        'Case #1: ' + _BAD_FORMAT_ERROR,
        FindError(
            None, self.input_file, self.output_file, """
Case #1: 0 0 0 -1
Case #2: 0 30 0 0
Case #3: 1 2 3 0
Case #4: 0 0 1 0
""".lstrip()))

    self.assertEqual(
        'Case #1: ' + _BAD_FORMAT_ERROR,
        FindError(
            None, self.input_file, self.output_file, """
Case #1: 0 0 0 1000000000
Case #2: 0 30 0 0
Case #3: 1 2 3 0
Case #4: 0 0 1 0
""".lstrip()))


def RunUnitTests():
  test = BrokenClockUnitTest()
  test_methods = [
      method_name for method_name in dir(test)
      if callable(getattr(test, method_name)) and method_name.startswith('test')
  ]
  for test_method in test_methods:
    print(test_method)
    getattr(test, test_method)()

  print('BrokenClockUnitTest passes.')
  sys.exit(0)


import sys
if __name__ == '__main__':
  if sys.argv[1] == '-2':
    RunUnitTests()

  result = FindError(None,
                     file(sys.argv[1]).read(),
                     file(sys.argv[3]).read(),
                     file(sys.argv[2]).read())
  if result:
    print >> sys.stderr, result
    sys.exit(1)
