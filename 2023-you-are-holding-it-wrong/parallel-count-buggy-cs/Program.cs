var counter = 0;

var t1 = Task.Run(() =>
{
    for (int i = 0; i < 50000; i++)
    {
        counter += 1;
    }
});

var t2 = Task.Run(() =>
{
    for (int i = 0; i < 50000; i++)
    {
        counter += 1;
    }
});

t1.Wait();
t2.Wait();

Console.WriteLine($"Result = {counter}");
