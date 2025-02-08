function setSmallSideMenu() {
    const openSmallSideMenu = document.getElementById('js-openSmallSideMenu') as HTMLButtonElement | null;
    if (openSmallSideMenu) {
        openSmallSideMenu.addEventListener('click', () => {
            const smallSideMenu = document.getElementById('js-smallSideMenu') as HTMLDivElement | null;
            if (smallSideMenu == null) {
                return;
            }
            smallSideMenu.classList.toggle('hidden');
        });
    }

    const closeSmallSideMenu = document.getElementById('js-closeSmallSideMenu') as HTMLButtonElement | null;
    if (closeSmallSideMenu) {
        closeSmallSideMenu.addEventListener('click', () => {
            const smallSideMenu = document.getElementById('js-smallSideMenu') as HTMLDivElement | null;
            if (smallSideMenu == null) {
                return;
            }
            smallSideMenu.classList.add('hidden');
        });
    }
}

export function setToggleMenuEvent() {
    setSmallSideMenu();
}