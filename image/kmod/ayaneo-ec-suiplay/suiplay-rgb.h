/* SPDX-License-Identifier: GPL-2.0+ */
/* Rare OS: the SuiPlay0X1's stick rings (suiplay-rgb.c). */
#ifndef SUIPLAY_RGB_H
#define SUIPLAY_RGB_H

struct device;

int suiplay_rgb_probe(struct device *dev);
void suiplay_rgb_suspend(struct device *dev);
void suiplay_rgb_resume(struct device *dev);
void suiplay_rgb_shutdown(struct device *dev);

#endif
