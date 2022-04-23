import os
import skimage
import skimage.io
import numpy as np
import warnings
import matplotlib.pyplot as plt
import pathlib
import math

image_input_dir = pathlib.Path("images/input/")
image_output_dir = pathlib.Path("images/output/")
os.makedirs(image_output_dir, exist_ok=True)


def read_image(imname: str, image_folder=image_input_dir) -> np.ndarray:
    """
    Reads image (imname) from folder image_folder
    """
    impath = image_folder.joinpath(imname)
    image = skimage.io.imread(impath)
    w, h = image.shape
    print(f"Reading image: {impath} ({w}x{h})")
    return image


def _save_im(imname: str, im: np.ndarray):
    """
    Saves image (im) to the directory image_output_dir with
    filename "imname"
    """
    im = to_uint8(im)
    impath = image_output_dir.joinpath(imname)
    print("Saving image to:", impath)
    skimage.io.imsave(impath, im, check_contrast=False)


def show_image(image, cmap: str = "gray", save_as: str = ""):
    if save_as != "":
        _save_im(save_as, image)
    show_images([image], cmap=cmap)


def show_images(
    images: "list[np.ndarray]", rows: int = 1, cmap: str = "gray", save_as: str = ""
):
    """Places all images in the list in a subplot figure, if "save_as" is set, the
    image will be saved in image_output_dir"""
    cols = math.ceil(len(images) / rows)
    size = 4
    plt.figure(figsize=(cols * size, rows * size))

    for i, image in enumerate(images):
        plt.subplot(rows, cols, i + 1)
        plt.imshow(image, cmap=cmap)
        plt.axis("off")

    plt.tight_layout()
    if save_as != "":
        path = image_output_dir.joinpath(save_as)
        print("Saving image to:", path)
        plt.savefig(path)
    plt.show()


def compare_images(
    original: np.ndarray, after: np.ndarray, filter_name: str, save_as=""
):
    fig, (ax1, ax2) = plt.subplots(ncols=2, figsize=(12, 6), sharex=True, sharey=True)
    ax1.imshow(original, cmap=plt.cm.gray)
    ax1.set_title("original")
    ax1.axis("off")
    ax2.imshow(after, cmap=plt.cm.gray)
    ax2.set_title(filter_name)
    ax2.axis("off")
    plt.tight_layout()
    if save_as != "":
        path = image_output_dir.joinpath(save_as)
        print("Saving image to:", path)
        plt.savefig(path)
    plt.show()


def to_uint8(im: np.ndarray) -> np.ndarray:
    """
    Converts and squashes an image to uint8.
    If image min/max is outside of [0.0, 1.0]
    We squash it to [0.0, 1.0]

    Args:
        im: np.ndarray of dtype np.uint8 or np.float
    returns:
        im: np.ndarray of dtype np.uint8 in range [0, 255]
    """
    if im.dtype == np.uint8:
        warnings.warn("Image is already np.uint8")
        return im
    if im.min() >= 0.0 and im.max() <= 1.0:
        im = (im * 255).astype(np.uint8)
        return im
    im = im - im.min()
    im = im / im.max()
    im = (im * 255).astype(np.uint8)
    return im


def uint8_to_float(im: np.array):
    """
    Converts an image from range 0-255 to 0-1

    Args:
        im: np.array
    Returns:
        im: np.array with same shape as np.array
    """
    if im.dtype == np.float32:
        warnings.warn("Image is already np.float32")
        return im
    im = im.astype(np.float32) / 255
    return im


def plot_histogram(histogram, threshold):
    plt.bar(range(0, 256), histogram, width=1)
    plt.vlines(threshold, ymin=0, ymax=max(histogram), colors="r")
    plt.show()
