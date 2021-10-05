using System;

static class QuestLogic
{
    public static bool CanFastAttack(bool knightIsAwake)
    {
        return !knightIsAwake;
    }

    public static bool CanSpy(bool knightIsAwake, bool archerIsAwake, bool prisonerIsAwake)
    {
        return knightIsAwake || archerIsAwake || prisonerIsAwake;
    }

    public static bool CanSignalPrisoner(bool archerIsAwake, bool prisonerIsAwake)
    {
        return prisonerIsAwake && !archerIsAwake;
    }

/*
Free prisoner: if the prisoner is awake and the other two characters are sleeping, 
a sneaky entry into the camp can free the prisoner.
This won't work if the prisoner is sleeping, as the prisoner will be startled by the sudden appearance of
her friend and the knight and archer will be awoken. The prisoner can also be freed if the archer is sleeping and Annalyn has her pet dog with her,
as the knight will be scared by the dog and will withdraw, and the archer can't equip his bow fast enough to prevent the prisoner from being freed.
*/
    public static bool CanFreePrisoner(bool knightIsAwake, bool archerIsAwake, bool prisonerIsAwake, bool petDogIsPresent)
    {
        return (prisonerIsAwake && !knightIsAwake && !archerIsAwake)
            || (!archerIsAwake && petDogIsPresent);
    }
}
