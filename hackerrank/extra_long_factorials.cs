// https://www.hackerrank.com/challenges/extra-long-factorials/problem
// Extra Long Factorials

using System.CodeDom.Compiler;
using System.Collections.Generic;
using System.Collections;
using System.ComponentModel;
using System.Diagnostics.CodeAnalysis;
using System.Globalization;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Runtime.Serialization;
using System.Text.RegularExpressions;
using System.Text;
using System.Numerics;
using System;

/*
Calculate and print the factorial of a given integer.
*/

class Result
{
    /*
     * Complete the 'extraLongFactorials' function below.
     *
     * The function accepts INTEGER n as parameter.
     */

    public static void extraLongFactorials(int n)
    {
      BigInteger result = new BigInteger(n);
      for (int i = n - 1; i > 0; i--) {
        result = result * (BigInteger)i;
      }
      Console.WriteLine(result);
    }

}

class Solution
{
    public static void Main(string[] args)
    {
        int n = Convert.ToInt32(Console.ReadLine().Trim());

        Result.extraLongFactorials(n);
    }
}
