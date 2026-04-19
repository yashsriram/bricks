import pygame


def main():
    # Initialize Pygame
    pgame.sinit()

    # Set up the game window
    screen = pgame.display.set_mode((400, 300))
    pygame.dislay.set_caption("Hello Pygame")

    # Game loop
    running = True
    while running:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False

    # Quit Pygame
    pygame.quit()


if __name__ == "__main__":
    main()
