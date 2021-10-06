using System;
using System.Linq;

public static class TelemetryBuffer
{
    public static byte[] ToBuffer(long reading)
    {
        byte[] bytes = BitConverter.GetBytes(reading);
        var res = new byte[9];

        var (prefix, num) = reading switch
        {
            (< int.MinValue or > uint.MaxValue) => (248, 8),
            (> int.MaxValue) => (4, 4),
            (< short.MinValue or > ushort.MaxValue) => (252, 4),
            (<= short.MaxValue) => (254, 2),
            _ => (2, 2)
        };

        res[0] = (byte)prefix;

        for(int i = 0; i < num; i ++) {
            res[i + 1] = bytes[i];
        }
        
        return res;
    }

    public static long FromBuffer(byte[] buffer)
    {
        long res = 0;
        res = buffer[0] switch {
            248 => BitConverter.ToInt64(buffer, 1),
            004 => BitConverter.ToUInt32(buffer, 1),
            252 => BitConverter.ToInt32(buffer, 1),
            002 => BitConverter.ToUInt16(buffer, 1),
            254 => BitConverter.ToInt16(buffer, 1),
            _ => 0
        };

        return res;
    }
}
