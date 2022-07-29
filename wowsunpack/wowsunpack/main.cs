using Newtonsoft.Json;
using Razorvine.Pickle;
using System;
using System.Collections;
using System.IO;
using System.IO.Compression;
using System.Linq;
using wowsunpack;

namespace HenryQuan
{
    class DataTable : Hashtable
    {
        public void __setstate__(Hashtable arg)
        {
            foreach (DictionaryEntry entry in arg)
            {
                this[entry.Key] = entry.Value;
            }
        }
    }

    class DataConstructor : IObjectConstructor
    {
        public object construct(object[] args)
        {
            return new DataTable();
        }
    }

    class WoWsUnpack
    {
        [DllExport]
        public static void unpack(string path, bool compact = false)
        {
            // record the time taken
            var start = DateTime.Now;
            Console.WriteLine("Unpacking...");
            // the raw data is in big endian, not little endian like Windows
            var rawData = File.ReadAllBytes(path);
            Console.WriteLine("Read GameParams.data");
            // reverse the raw data and remove first two bytes (zlib header)
            var data = rawData.Reverse().Skip(2).ToArray();
            Console.WriteLine("Reversed GameParams.data");

            using (var stream = new MemoryStream(data))
            {
                // decompress zlib
                using (var decompressed = new DeflateStream(stream, CompressionMode.Decompress))
                {
                    Console.WriteLine("Decompressed GameParams.data");
                    var pickler = new Unpickler();
                    Unpickler.registerConstructor("copy_reg", "_reconstructor", new DataConstructor());
                    object[] unpacked = (object[])pickler.load(decompressed);
                    Console.WriteLine("Decoded GameParams.data");

                    var settings = new JsonSerializerSettings();
                    settings.Formatting = Formatting.Indented;
                    var serializer = JsonSerializer.Create(settings);
                    using (var writer = new StreamWriter(@"GameParams.json"))
                    {
                        serializer.Serialize(writer, unpacked[0]);
                    }

                    var end = DateTime.Now;
                    Console.WriteLine("Time taken: {0}", end - start);
                }
            }
        }
    }
}
