#include <stdio.h>
#include <stdlib.h>
#include <time.h>

int main()
{
    int hours = 12;
    int stand = 0;
    int who = 0;
    printf("WELCOME, USER. CURRENT SYSTEM TIME:\n");
    //SHOW SYSTEM TIME
    time_t mytime;
    mytime = time(NULL);
    printf(ctime(&mytime));
    printf("REMOTE LINK SUCCESSFUL.\n=====================================\nBEGIN:\n\n");

    //INTRODUCTION
    CHOICE1: printf("You open your eyes. You feel the dewy grass and a light breeze against your skin. \nYou're on your back, facing a bright, scintillating sky. \n \nWelcome to consciousness. Your stay will expire in %d hours.\n", hours);
    int choice;
    printf("\nWhat next? \n");
    printf("1. Remain where I am.\n");
    printf("2. Stand up. \n");

    scanf("%d", &choice);

//LAYING STORYLINE
    if (choice == 1) {
        CHOICE2: hours = hours-3;
        printf("-------------------------------------------------------\n\n");
        printf("You remain where you are. Laying perfectly still, it almost feels as if you could fall into the blue expanse above you.\nYou watch as the sun slowly creeps across the sky, edging softly toward the horizon.\nIf you were human, this would be a great way to lose your eyesight.\nHowever, your visual sensors are unaffected.\n");
        printf("\n%d hours now remain.\n", hours);
        //Decision A
            int choiceA;
            printf("\nWhat next? \n");
            printf("1. Who am I? \n");
            printf("2. Stand up. \n");
            scanf("%d", &choiceA);

        //Choices
            if (choiceA == 1)
            {
                        WHO: hours = hours-2;
                        if (who == 0){
                            printf("-------------------------------------------------------\n\n");
                            printf("This isn't an easy question to answer, and many conscious organisms will struggle with this idea.\nThe fact that you're asking this is heartening to me as Lead Roboticist.\nYou might just be the most incredible thing I've ever created.\n");
                            }
                        if (who == 1){
                            printf("-------------------------------------------------------\n\n");
                            printf("I realised early on that I couldn't create synthetic intelligence without also making you alive. You cannot remove intelligence from its context without creating a mere simulacrum.\nYou, however, are the real thing. A completely new life form.\nI'm no woman of God, but I've decided to call you Eve, despite you being technically genderless.\n");
                        }
                        printf("\n%d hours now remain.\n", hours);
                        who = 1;

                        //Decision B

                        int choiceB;
                        printf("\nWhat next? \n");
                        printf("1. Why am I here? \n");
                            if (stand == 0){
                                printf("2. Stand up. \n");
                            } if (stand == 1){
                                printf("2. Take some steps.");
                            }

                        scanf("%d", &choiceB);

                        //Choices
                            if (choiceB == 1){
                                WHY: hours = hours-1;
                                printf("-------------------------------------------------------\n\n");
                                printf("I thought long and hard about bringing you into existence, especially given your... time constraint.\nIn the end, I figured it would be better for you to experience this phenomenon, just for a short while, than never to experience it at all.\nBut in truth, you're only here because I had the ability to bring you about. Perhaps it was selfish of me.\n");
                                printf("\n%d hours now remain.\n", hours);

                                //Decision C
                                int choiceC;
                                printf("\nWhat next? \n");
                                printf("1. Am I alone? \n");
                                scanf("%d", &choiceC);

                                //Choices
                                if (choiceC == 1){
                                    hours = hours-2;
                                    printf("-------------------------------------------------------\n\n");
                                    printf("You're the first of your kind, yes.\nI feel as though you may also be the last.\nYou're the result of years of algorithmic toil and mechanical experimentation, however you've opted not to make any use of your body during this experiment.\nIt's yours, so please don't feel guilty. As your creator, it's a little difficult to now let go of the control, but I need to let this be your experience.\n \n \n ==========================\n");
                                    printf("EPILOGUE:\n In your final hour, you watch as the sun finally leaves your field of vision.\n In its wake, the sky darkens, creating a beautiful deep gradient.\n Finally, you close your eyes one last time, and a warm static envelopes your senses.\n \nTHE END.\n");
                                }
                                    else {
                                        printf("REMOTE LINK ERROR: USER INPUT INVALID. TRY AGAIN.\n");
                                        goto WHY;
                                    }
                            }

                            if (choiceB == 2 && stand == 0){
                                goto STAND;
                            }
                            if (choiceB == 2 && stand == 1) {
                                goto WALK;
                            }
                            else{
                                printf("REMOTE LINK ERROR: USER INPUT INVALID. TRY AGAIN.\n");
                                goto WHO;
                            }

            }

            if (choiceA == 2){
                    goto STAND;
            }
            else {
                    printf("REMOTE LINK ERROR: USER INPUT INVALID. TRY AGAIN.\n");
                    goto CHOICE2;
            }
    }

 //STANDING STORYLINE
    if (choice == 2) {
                    STAND: hours = hours - 1;
                    stand = 1;
                    printf("-------------------------------------------------------\n\n");
                    printf("You rise slowly to your knees, shakily at first, but slowly gaining your stability as your gyroscope springs into operation.\nYou look down at your limbs: two long appendages with elbow joints, wrists and hands.\nYou brace them against the grass below you and rise slowly to your feet.\n");
                    printf("\n%d hours now remain.\n", hours);
                    //Decision A
                        int choiceA;
                        printf("\nWhat next? \n");
                            if (who == 0){
                                printf("1. Who am I? \n");
                            }
                            if (who == 1){
                                printf("1. I'd like to know who I am.\n");
                            }
                            printf("2. Take a few steps. \n");
                        scanf("%d", &choiceA);

                        if (choiceA == 1)
                        {
                            goto WHO;
                        }

                        if (choiceA == 2){
                            hours = hours-2;
                            printf("-------------------------------------------------------\n\n");
                            printf("As you take your first cursory steps, you feel the grass lap gently against the bottoms of your feet.\nYou enjoy the sound it creates: a barely-audible rustle, with a satisfying soft crunch on each step.\nYou look into the distance and notice the vegetation and its vivid green hue.\n");
                            printf("\n%d hours now remain.\n", hours);

                            //Decision B

                            int choiceB;
                            printf("\nWhat next? \n");
                            printf("1. Why am I here? \n");
                            printf("2. Keep walking. \n");
                            scanf("%d", &choiceB);

                            //Choices
                                if (choiceB == 1){
                                    goto WHY;
                                }
                                if (choiceB == 2){
                                    WALK: hours = hours-3;
                                    printf("-------------------------------------------------------\n\n");
                                    printf("Walking has begun to feel almost natural, requiring less effort with each step.\nYou feel your environment opening up to you; the breeze envelopes your entire body. Suddenly, you pause. You hear a loud, shrill call coming from a nearby tree.\nA sensation washes over you; filling you with conflicting desires to flee or defend yourself.\nThe sound's creator flies out of the tree and away in a flurry of flaps and squawks.\nIt is small, and you realise it poses no threat. However, the shock has left your energy reserves drained.\n");
                                    printf("\n%d hours now remain.\n", hours);

                                    //Decision C
                                    int choiceC;
                                    printf("\nWhat next? \n");
                                    printf("1. Sit and rest. \n");
                                    scanf("%d", &choiceC);

                                    //Choices
                                    if (choiceC == 1){
                                        hours = hours-2;
                                        printf("-------------------------------------------------------\n\n");
                                        printf("You slowly lower yourself to the ground. Once seated, you can feel your energy slowly begin to restore. A variety of small life-forms crawling in the grass find their way to your skin, lightly tickling your sensors.\n\n \n ==========================\n");
                                        printf("EPILOGUE:\n As you watch the sun make its final descent, you realise how little you know about yourself and your strange, temporary world. However, you have now experienced the phenomenon of consciousness; making use of all its capabilities. A warm static overcomes you. \n \n \nTHE END.\n");

                                    }
                                    else{
                                        printf("REMOTE LINK ERROR: USER INPUT INVALID. TRY AGAIN.\n");
                                        goto WALK;
                                    }
                                }
                        }
                        else {
                            printf("REMOTE LINK ERROR: USER INPUT INVALID. TRY AGAIN.\n");
                            goto STAND;
                        }
                }
        else {
            printf("REMOTE LINK ERROR: USER INPUT INVALID. TRY AGAIN.\n");
            goto CHOICE1;
        }

    system("pause");
    return 0;
}