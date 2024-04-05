using System;
using Pokecount_csharp.API.Models;

namespace Pokecount_csharp.API.logic
{
    public class PokemonComputing
    {
        public async Task<List<Pokemon>> ComputePokemonList(List<PokeElement> pokeList)
        {
            List<Pokemon> pokedex = new();
            for(int i = 0; i < 100_000; i++)
            {
                int pokedexNumber = 1;
                foreach (var item in pokeList)
                {
                    pokedex.Add(new(){
                        Name = item.Name,
                        Number = pokedexNumber
                    });
                    pokedexNumber++;
                }
            }            
            return pokedex;
        }
    }
}
