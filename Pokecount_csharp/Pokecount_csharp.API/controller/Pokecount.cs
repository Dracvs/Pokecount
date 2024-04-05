using System;
using Microsoft.AspNetCore.Mvc;
using Pokecount_csharp.API.logic;
using Pokecount_csharp.API.Models;

namespace Pokecount_csharp.API.controller
{
    [Route("api/[controller]")]
    [ApiController]
    public class Pokecount : ControllerBase
    {
        private readonly PokemonComputing _pokemonComputing = new();

        [HttpPost]
        [Route("ComputePokemon")]
        public async Task<IActionResult> ComputePokemon(List<PokeElement> pokeList)
        {
            var watch = new System.Diagnostics.Stopwatch();
            watch.Start();
            var list = await _pokemonComputing.ComputePokemonList(pokeList);
            watch.Stop();
            GC.Collect();
            return Ok($"CPU Execution time: {watch.ElapsedMilliseconds} ms");
        }
    }
}
